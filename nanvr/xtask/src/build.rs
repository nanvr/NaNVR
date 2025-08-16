use crate::CommonBuildFlags;
use clap::ValueEnum;
use filepaths::Layout;
use shared::NANVR_HIGH_NAME;
use shared::NANVR_LOW_NAME;
use std::{
    env,
    fmt::{self, Display, Formatter},
    fs,
    path::PathBuf,
    vec,
};
use xshell::{Shell, cmd};

#[derive(Clone, Copy, Default, ValueEnum)]
pub enum Profile {
    #[default]
    Debug,
    Release,
    Distribution,
}

impl Display for Profile {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let string = match self {
            Profile::Distribution => "distribution",
            Profile::Release => "release",
            Profile::Debug => "debug",
        };
        write!(f, "{string}")
    }
}

pub fn build_server_lib(
    profile: Profile,
    root: Option<String>,
    common_build_flags: CommonBuildFlags,
) {
    let sh = Shell::new().unwrap();

    let mut flags = vec![];
    match profile {
        Profile::Distribution => {
            flags.push("--profile");
            flags.push("distribution");
        }
        Profile::Release => flags.push("--release"),
        Profile::Debug => (),
    }
    if common_build_flags.locked {
        flags.push("--locked");
    }
    if common_build_flags.offline {
        flags.push("--offline");
    }
    let flags_ref = &flags;

    let artifacts_dir = filepaths::target_dir().join(profile.to_string());

    let build_dir = filepaths::build_dir().join("server_core");
    sh.create_dir(&build_dir).unwrap();

    if let Some(root) = root {
        sh.set_var(format!("{NANVR_HIGH_NAME}_ROOT_DIR"), root);
    }

    let _push_guard = sh.push_dir(filepaths::crate_dir("server_core"));
    cmd!(sh, "cargo build {flags_ref...}").run().unwrap();

    sh.copy_file(
        artifacts_dir.join(filepaths::dynlib_fname("server_core")),
        &build_dir,
    )
    .unwrap();

    let out = build_dir.join("server_core.h");
    cmd!(sh, "cbindgen --output {out}").run().unwrap();
}

pub fn build_streamer(
    profile: Profile,
    root: Option<String>,
    common_build_flags: CommonBuildFlags,
    keep_config: bool,
) {
    let sh = Shell::new().unwrap();

    let build_layout = Layout::new(&filepaths::streamer_build_dir());

    let mut common_flags = vec![];
    match profile {
        Profile::Distribution => {
            common_flags.push("--profile");
            common_flags.push("distribution");
        }
        Profile::Release => common_flags.push("--release"),
        Profile::Debug => (),
    }
    if common_build_flags.locked {
        common_flags.push("--locked");
    }
    if common_build_flags.frozen {
        common_flags.push("--frozen");
    }
    if common_build_flags.offline {
        common_flags.push("--offline");
    }

    let artifacts_dir = filepaths::target_dir().join(profile.to_string());

    let common_flags_ref = &common_flags;

    let maybe_config = if keep_config {
        fs::read_to_string(build_layout.session()).ok()
    } else {
        None
    };

    sh.remove_path(filepaths::streamer_build_dir()).ok();
    sh.create_dir(build_layout.openvr_driver_lib_dir()).unwrap();
    sh.create_dir(&build_layout.executables_dir).unwrap();

    if let Some(config) = maybe_config {
        fs::write(build_layout.session(), config).ok();
    }

    if let Some(root) = root {
        sh.set_var(format!("{NANVR_HIGH_NAME}_ROOT_DIR"), root);
    }

    // build server
    {
        let profiling_flag = if common_build_flags.profiling {
            vec!["--features", "server_core/trace-performance"]
        } else {
            vec![]
        };

        let _push_guard = sh.push_dir(filepaths::crate_dir("server_openvr"));
        cmd!(sh, "cargo build {common_flags_ref...} {profiling_flag...}")
            .run()
            .unwrap();

        sh.copy_file(
            artifacts_dir.join(filepaths::dynlib_fname("server_openvr")),
            build_layout.openvr_driver_lib(),
        )
        .unwrap();
    }

    // Build dashboard
    {
        let _push_guard = sh.push_dir(filepaths::crate_dir("dashboard"));
        cmd!(sh, "cargo build {common_flags_ref...}").run().unwrap();

        sh.copy_file(
            artifacts_dir.join(filepaths::exec_fname("dashboard")),
            build_layout.dashboard_exe(),
        )
        .unwrap();
    }

    // build compositor wrapper
    let _push_guard = sh.push_dir(filepaths::crate_dir("vrcompositor_wrapper"));
    cmd!(sh, "cargo build {common_flags_ref...}").run().unwrap();
    sh.create_dir(&build_layout.vrcompositor_wrapper_dir)
        .unwrap();
    sh.copy_file(
        artifacts_dir.join("vrcompositor_wrapper"),
        build_layout.vrcompositor_wrapper(),
    )
    .unwrap();
    sh.copy_file(
        artifacts_dir.join(format!("{NANVR_LOW_NAME}_drm_lease_shim.so")),
        build_layout.drm_lease_shim(),
    )
    .unwrap();

    // build vulkan layer
    let _push_guard = sh.push_dir(filepaths::crate_dir("vulkan_layer"));
    cmd!(sh, "cargo build {common_flags_ref...}").run().unwrap();
    sh.create_dir(&build_layout.libraries_dir).unwrap();
    sh.copy_file(
        artifacts_dir.join(filepaths::dynlib_fname("vulkan_layer")),
        build_layout.vulkan_layer(),
    )
    .unwrap();

    // copy vulkan layer manifest
    sh.create_dir(&build_layout.vulkan_layer_manifest_dir)
        .unwrap();
    sh.copy_file(
        filepaths::crate_dir("vulkan_layer").join(format!("layer/{NANVR_LOW_NAME}_x86_64.json")),
        build_layout.vulkan_layer_manifest(),
    )
    .unwrap();

    sh.copy_file(
        filepaths::workspace_dir().join("openvr/bin/linux64/libopenvr_api.so"),
        build_layout.openvr_driver_lib_dir(),
    )
    .unwrap();

    let firewall_script =
        filepaths::crate_dir("xtask").join(format!("firewall/{NANVR_LOW_NAME}_fw_config.sh"));
    let firewalld =
        filepaths::crate_dir("xtask").join(format!("firewall/{NANVR_LOW_NAME}-firewalld.xml"));
    let ufw = filepaths::crate_dir("xtask").join(format!("firewall/ufw-{NANVR_LOW_NAME}"));

    // copy linux specific firewalls
    sh.copy_file(firewall_script, build_layout.firewall_script())
        .unwrap();
    sh.copy_file(firewalld, build_layout.firewalld_config())
        .unwrap();
    sh.copy_file(ufw, build_layout.ufw_config()).unwrap();

    // copy static resources
    {
        // copy driver manifest
        sh.copy_file(
            filepaths::crate_dir("xtask").join("resources/driver.vrdrivermanifest"),
            build_layout.openvr_driver_manifest(),
        )
        .unwrap();
    }
}

pub fn build_launcher(profile: Profile, common_build_flags: &CommonBuildFlags) {
    let sh = Shell::new().unwrap();

    let mut common_flags = vec![];
    match profile {
        Profile::Distribution => {
            common_flags.push("--profile");
            common_flags.push("distribution");
        }
        Profile::Release => common_flags.push("--release"),
        Profile::Debug => (),
    }
    if common_build_flags.locked {
        common_flags.push("--locked");
    }
    if common_build_flags.offline {
        common_flags.push("--offline");
    }
    let common_flags_ref = &common_flags;

    sh.create_dir(filepaths::launcher_build_dir()).unwrap();

    cmd!(sh, "cargo build -p launcher {common_flags_ref...}")
        .run()
        .unwrap();

    sh.copy_file(
        filepaths::target_dir()
            .join(profile.to_string())
            .join(filepaths::exec_fname("launcher")),
        filepaths::launcher_build_exe_path(),
    )
    .unwrap();
}

fn build_android_lib_impl(dir_name: &str, profile: Profile, link_stdcpp: bool, all_targets: bool) {
    let sh = Shell::new().unwrap();

    let mut ndk_flags = vec!["--no-strip", "-p", "26", "-t", "arm64-v8a"];

    if all_targets {
        ndk_flags.extend(["-t", "armeabi-v7a", "-t", "x86_64", "-t", "x86"]);
    }

    let mut rust_flags = vec![];
    match profile {
        Profile::Distribution => {
            rust_flags.push("--profile");
            rust_flags.push("distribution");
        }
        Profile::Release => rust_flags.push("--release"),
        Profile::Debug => (),
    }
    if !link_stdcpp {
        rust_flags.push("--no-default-features");
    }
    let rust_flags_ref = &rust_flags;

    let build_dir = filepaths::build_dir().join(format!("{NANVR_LOW_NAME}_{dir_name}"));
    sh.create_dir(&build_dir).unwrap();

    let _push_guard = sh.push_dir(filepaths::crate_dir(dir_name));
    cmd!(
        sh,
        "cargo ndk {ndk_flags...} -o {build_dir} build {rust_flags_ref...}"
    )
    .run()
    .unwrap();

    let out = build_dir.join(format!("{NANVR_LOW_NAME}_{dir_name}.h"));
    cmd!(sh, "cbindgen --output {out}").run().unwrap();
}

pub fn build_android_client_core_lib(profile: Profile, link_stdcpp: bool, all_targets: bool) {
    build_android_lib_impl("client_core", profile, link_stdcpp, all_targets);
}

pub fn build_android_client_openxr_lib(profile: Profile, link_stdcpp: bool) {
    build_android_lib_impl("client_openxr", profile, link_stdcpp, false);
}

pub fn build_android_client(profile: Profile) {
    let artifact_name: String = format!("{NANVR_LOW_NAME}_client_android");
    let sh = Shell::new().unwrap();

    let mut flags = vec![];
    match profile {
        Profile::Distribution => {
            flags.push("--profile");
            flags.push("distribution");
        }
        Profile::Release => flags.push("--release"),
        Profile::Debug => (),
    }
    let flags_ref = &flags;

    let target_dir = filepaths::target_dir();
    let build_dir = filepaths::build_dir().join(&artifact_name);
    sh.create_dir(&build_dir).unwrap();

    // Create debug keystore (signing will be overwritten by CI)
    if env::var(format!(
        "CARGO_APK_{}_KEYSTORE",
        profile.to_string().to_uppercase()
    ))
    .is_err()
        && matches!(profile, Profile::Release | Profile::Distribution)
    {
        let keystore_path = build_dir.join("debug.keystore");
        if !keystore_path.exists() {
            let keytool = PathBuf::from(env::var("JAVA_HOME").expect("Env var JAVA_HOME not set"))
                .join("bin")
                .join(filepaths::exec_fname("keytool"));
            let pass = "nanvrclient";

            let other = vec![
                "-genkey",
                "-v",
                "-alias",
                "androiddebugkey",
                "-dname",
                "CN=Android Debug,O=Android,C=US",
                "-keyalg",
                "RSA",
                "-keysize",
                "2048",
                "-validity",
                "10000",
            ];

            cmd!(
                sh,
                "{keytool} -keystore {keystore_path} -storepass {pass} -keypass {pass} {other...}"
            )
            .run()
            .unwrap();
        }
    }

    let _push_guard = sh.push_dir(filepaths::crate_dir("client_openxr"));
    cmd!(
        sh,
        "cargo apk build --target-dir={target_dir} {flags_ref...}"
    )
    .run()
    .unwrap();

    sh.copy_file(
        filepaths::target_dir()
            .join(profile.to_string())
            .join("apk/client_openxr.apk"),
        build_dir.join(format!("{artifact_name}.apk")),
    )
    .unwrap();
}
