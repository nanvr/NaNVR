mod dashboard;
mod data_sources;
mod logging_backend;
mod startup_checks;
mod steamvr_launcher;

use dashboard::Dashboard;
use data_sources::DataSources;
use egui_i18n::tr;

fn get_filesystem_layout() -> filepaths::Layout {
    filepaths::filesystem_layout_from_dashboard_exe(&std::env::current_exe().unwrap()).unwrap()
}

fn init_i18n() {
    let en_us = String::from_utf8_lossy(include_bytes!("../resources/i18n/en_US.egl"));
    egui_i18n::load_translations_from_text("en_US", en_us).unwrap();
    egui_i18n::set_language("en_US");
    egui_i18n::set_fallback("en_US");
}

fn main() {
    use eframe::{
        NativeOptions,
        egui::{IconData, ViewportBuilder},
    };

    use ico::IconDir;
    use shared::info;
    use shared::{NANVR_LOW_NAME, NANVR_NAME, NANVR_VERSION};
    use std::{env, ffi::OsStr, fs};
    use std::{io::Cursor, sync::mpsc};

    let (server_events_sender, server_events_receiver) = mpsc::channel();
    logging_backend::init_logging(server_events_sender.clone());

    // Kill any other dashboard instance
    let self_path = std::env::current_exe().unwrap();
    for proc in
        sysinfo::System::new_all().processes_by_name(OsStr::new(&filepaths::dashboard_fname()))
    {
        if let Some(other_path) = proc.exe()
            && other_path != self_path
        {
            info!(
                "{}",
                tr!("killing-other-dashboard", {path: other_path.display()})
            );
            proc.kill();
        }
    }
    startup_checks::hardware_checks();
    startup_checks::audio_check();

    data_sources::clean_session();

    if data_sources::get_read_only_local_session()
        .settings()
        .extra
        .steamvr_launcher
        .open_close_steamvr_with_dashboard
    {
        steamvr_launcher::LAUNCHER.lock().launch_steamvr()
    }

    let ico = IconDir::read(Cursor::new(include_bytes!("../resources/dashboard.ico"))).unwrap();
    let image = ico.entries().first().unwrap().decode().unwrap();

    // Workaround for the steam deck
    if fs::read_to_string("/sys/devices/virtual/dmi/id/board_vendor")
        .map(|vendor| vendor.trim() == "Valve")
        .unwrap_or(false)
    {
        unsafe { env::set_var("WINIT_X11_SCALE_FACTOR", "1") };
    }

    init_i18n();

    eframe::run_native(
        &format!(
            "{NANVR_NAME} Dashboard (streamer {})",
            NANVR_VERSION.to_owned()
        ),
        NativeOptions {
            viewport: ViewportBuilder::default()
                .with_app_id(format!("{NANVR_LOW_NAME}.dashboard"))
                .with_inner_size((900.0, 600.0))
                .with_icon(IconData {
                    rgba: image.rgba_data().to_owned(),
                    width: image.width(),
                    height: image.height(),
                }),
            centered: true,
            ..Default::default()
        },
        {
            Box::new(move |creation_context| {
                let data_source = DataSources::new(
                    creation_context.egui_ctx.clone(),
                    server_events_sender,
                    server_events_receiver,
                );

                Ok(Box::new(Dashboard::new(creation_context, data_source)))
            })
        },
    )
    .unwrap();
}
