use crate::data_sources;

use const_format::formatcp;
use serde_json::{self, json};
use shared::{
    NANVR_LOW_NAME, NANVR_NAME,
    anyhow::{Context, Result, bail},
    debug,
    glam::bool,
    parking_lot::Mutex,
    warn,
};
use std::{
    ffi::OsStr,
    fs,
    marker::PhantomData,
    process::Command,
    thread,
    time::{Duration, Instant},
};
use sysinfo::{Process, ProcessesToUpdate, System};
use wired::commands as adb;

const SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(10);
const DRIVER_KEY: &str = formatcp!("driver_{NANVR_LOW_NAME}_server");
const BLOCKED_KEY: &str = "blocked_by_safe_mode";

// Singleton with exclusive access
pub static LAUNCHER: Mutex<Launcher> = Mutex::new(Launcher {
    _phantom: PhantomData,
});

pub struct Launcher {
    _phantom: PhantomData<()>,
}

impl Launcher {
    pub fn launch_steamvr(&self) {
        // The ADB server might be left running because of a unclean termination of SteamVR
        // Note that this will also kill a system wide ADB server not started by NaNVR
        let wired_enabled = data_sources::get_read_only_local_session()
            .session()
            .client_connections
            .contains_key(net_sockets::WIRED_CLIENT_HOSTNAME);
        if wired_enabled && let Some(path) = adb::get_adb_path(&crate::get_filesystem_layout()) {
            adb::kill_server(&path).ok();
        }

        let nanvr_driver_dir = crate::get_filesystem_layout().openvr_driver_root_dir;

        // Make sure to unregister any other NaNVR driver because it would cause a socket conflict
        let other_nanvr_dirs = server_io::get_registered_drivers()
            .unwrap_or_default()
            .into_iter()
            .filter(|path| {
                path.to_string_lossy()
                    .to_lowercase()
                    .contains(NANVR_LOW_NAME)
                    && *path != nanvr_driver_dir
            })
            .collect::<Vec<_>>();
        server_io::driver_registration(&other_nanvr_dirs, false).ok();

        server_io::driver_registration(&[nanvr_driver_dir], true).ok();

        if let Err(err) = unblock_nanvr_driver() {
            warn!("Failed to unblock {NANVR_NAME} driver: {:?}", err);
        }

        let vrcompositor_wrap_result = maybe_wrap_vrcompositor_launcher();
        shared::show_err(maybe_wrap_vrcompositor_launcher());
        if vrcompositor_wrap_result.is_err() {
            return;
        }

        if !is_steamvr_running() {
            debug!("SteamVR is dead. Launching...");

            start_steamvr();
        }
    }

    pub fn ensure_steamvr_shutdown(&self) {
        debug!("Waiting for SteamVR to shutdown...");
        let start_time = Instant::now();
        while start_time.elapsed() < SHUTDOWN_TIMEOUT && is_steamvr_running() {
            thread::sleep(Duration::from_millis(500));
        }

        maybe_kill_steamvr();
    }

    pub fn restart_steamvr(&self) {
        self.ensure_steamvr_shutdown();
        self.launch_steamvr();
    }
}

fn is_steamvr_running() -> bool {
    System::new_all()
        .processes_by_name(OsStr::new(&filepaths::exec_fname("vrserver")))
        .count()
        != 0
}

fn maybe_kill_steamvr() {
    let mut system = System::new_all();

    #[allow(unused_variables)]
    for process in system.processes_by_name(OsStr::new(&filepaths::exec_fname("vrmonitor"))) {
        debug!("Killing vrmonitor");

        terminate_process(process);

        thread::sleep(Duration::from_secs(1));
    }

    system.refresh_processes(ProcessesToUpdate::All, true);

    #[allow(unused_variables)]
    for process in system.processes_by_name(OsStr::new(&filepaths::exec_fname("vrserver"))) {
        debug!("Killing vrserver");

        terminate_process(process);

        thread::sleep(Duration::from_secs(1));
    }
}

fn unblock_nanvr_driver() -> Result<()> {
    let path = server_io::steamvr_settings_file_path()?;
    let text = fs::read_to_string(&path).with_context(|| format!("Failed to read {path:?}"))?;
    let new_text = unblock_nanvr_driver_within_vrsettings(text.as_str())
        .with_context(|| "Failed to rewrite .vrsettings.")?;
    fs::write(&path, new_text)
        .with_context(|| "Failed to write .vrsettings back after changing it.")?;
    Ok(())
}

// Reads and writes back steamvr.vrsettings in order to
// ensure the NaNVR driver is not blocked (safe mode).
fn unblock_nanvr_driver_within_vrsettings(text: &str) -> Result<String> {
    let mut settings = serde_json::from_str::<serde_json::Value>(text)?;
    let values = settings
        .as_object_mut()
        .with_context(|| "Failed to parse .vrsettings.")?;
    let blocked = values
        .get(DRIVER_KEY)
        .and_then(|driver| driver.get(BLOCKED_KEY))
        .and_then(|blocked| blocked.as_bool())
        .unwrap_or(false);

    if blocked {
        debug!("Unblocking {NANVR_NAME} driver in SteamVR.");
        if !values.contains_key(DRIVER_KEY) {
            values.insert(DRIVER_KEY.into(), json!({}));
        }
        let driver = settings[DRIVER_KEY]
            .as_object_mut()
            .with_context(|| format!("Did not find {NANVR_NAME} key in settings."))?;
        driver.insert(BLOCKED_KEY.into(), json!(false)); // overwrites if present
    } else {
        debug!("{NANVR_NAME} is not blocked in SteamVR.");
    }

    Ok(serde_json::to_string_pretty(&settings)?)
}

fn start_steamvr() {
    Command::new("steam")
        .args(["steam://rungameid/250820"])
        .spawn()
        .ok();
}

fn terminate_process(process: &Process) {
    process.kill_with(sysinfo::Signal::Term);
}

fn maybe_wrap_vrcompositor_launcher() -> shared::anyhow::Result<()> {
    let steamvr_bin_dir = server_io::steamvr_root_dir()?.join("bin").join("linux64");
    let steamvr_vrserver_path = steamvr_bin_dir.join("vrserver");
    debug!(
        "File path used to check for linux files: {}",
        steamvr_vrserver_path.display().to_string()
    );
    match steamvr_vrserver_path.try_exists() {
        Ok(exists) => {
            if !exists {
                bail!(
                    "SteamVR Linux files missing, aborting startup, please re-check compatibility tools for SteamVR, verify integrity of files for SteamVR and make sure you're not using Flatpak Steam with non-Flatpak {NANVR_NAME}."
                );
            }
        }
        Err(e) => {
            return Err(e.into());
        }
    };

    let launcher_path = steamvr_bin_dir.join("vrcompositor");
    // In case of SteamVR update, vrcompositor will be restored
    if fs::read_link(&launcher_path).is_ok() {
        fs::remove_file(&launcher_path)?; // recreate the link
    } else {
        fs::rename(&launcher_path, steamvr_bin_dir.join("vrcompositor.real"))?;
    }

    std::os::unix::fs::symlink(
        crate::get_filesystem_layout().vrcompositor_wrapper(),
        &launcher_path,
    )?;

    Ok(())
}
