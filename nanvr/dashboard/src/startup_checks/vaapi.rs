use egui_i18n::tr;
use libva::{VAEntrypoint, VAProfile};
use shared::{error, info};

pub fn encoder_check() {
    if let Some(libva_display) = libva::Display::open() {
        if let Ok(vendor_string) = libva_display.query_vendor_string() {
            info!(
                "{}",
                tr!("dashboard-encoder-notice", {vendor_string: vendor_string, newline: "\n"})
            );
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
        shared::show_e(tr!("no-vaapi-runtime", {newline: "\n"}));
    }
}

fn probe_libva_encoder_profile(
    libva_display: &std::rc::Rc<libva::Display>,
    profile_type: VAProfile::Type,
    profile_name: &str,
    is_critical: bool,
) {
    let profile_probe = libva_display.query_config_entrypoints(profile_type);

    let message = match profile_probe {
        Ok(profile) => {
            if !profile.contains(&VAEntrypoint::VAEntrypointEncSlice) {
                Some(tr!("no-encoding-with-encoder", {profile_name: profile_name}))
            } else {
                None
            }
        }
        Err(_) => Some(tr!("couldnt-find-encoder", {profile_name: profile_name})),
    };
    if let Some(message) = message {
        if is_critical {
            error!("{}", message);
        } else {
            info!("{}", message);
        }
    }
}
