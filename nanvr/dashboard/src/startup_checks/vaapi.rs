use libva::{VAEntrypoint, VAProfile};
use shared::{error, info};

pub fn encoder_check() {
    if let Some(libva_display) = libva::Display::open() {
        if let Ok(vendor_string) = libva_display.query_vendor_string() {
            info!("Dashboard (System) GPU Encoder: {}", vendor_string);
            info!("Note that, this GPU Encoder may differ from the one used by SteamVR");
        }
        probe_libva_encoder_profile(&libva_display, VAProfile::VAProfileH264Main, "H264", true);
        probe_libva_encoder_profile(&libva_display, VAProfile::VAProfileHEVCMain, "HEVC", true);
        probe_libva_encoder_profile(
            &libva_display,
            VAProfile::VAProfileAV1Profile0,
            "AV1",
            false,
        );
    } else {
        shared::show_e(
            "Couldn't find encoder (VA-API) runtime on system. \
Please install encoder (VA-API) runtime for your distribution in order for hardware encoding to work.",
        );
    }
}

fn probe_libva_encoder_profile(
    libva_display: &std::rc::Rc<libva::Display>,
    profile_type: VAProfile::Type,
    profile_name: &str,
    is_critical: bool,
) {
    let profile_probe = libva_display.query_config_entrypoints(profile_type);
    let mut message = String::new();
    if profile_probe.is_err() {
        message = format!("Couldn't find {profile_name} encoder.");
    } else if let Ok(profile) = profile_probe {
        if profile.is_empty() {
            message = format!("{profile_name} profile entrypoint is empty.");
        } else if !profile.contains(&VAEntrypoint::VAEntrypointEncSlice) {
            message = format!("{profile_name} profile does not contain encoding entrypoint.");
        }
    }
    if !message.is_empty() {
        if is_critical {
            error!("{} Your gpu does not suport encoding with this.", message);
        } else {
            info!(
                "{}
                Your gpu does not suport encoding with this. \
            If you're not using this encoder, ignore this message.",
                message
            );
        }
    }
}
