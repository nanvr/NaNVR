use crate::BuildPlatform;

pub mod android;
pub mod linux;
pub enum OpenXRLoadersSelection {
    OnlyGeneric,
    OnlyPico,
    All,
}

pub fn prepare_server_deps(platform: Option<BuildPlatform>, enable_nvenc: bool) {
    match platform {
        Some(BuildPlatform::Linux) => linux::prepare_deps(enable_nvenc),
        Some(BuildPlatform::Android) => panic!("Android is not supported"),
        None => {
            if cfg!(target_os = "linux") {
                linux::prepare_deps(enable_nvenc);
            } else {
                panic!("Unsupported platform");
            }
        }
    }
}

pub fn download_server_deps(platform: Option<BuildPlatform>, enable_nvenc: bool) {
    match platform {
        Some(BuildPlatform::Linux) => linux::download_deps(enable_nvenc),
        Some(BuildPlatform::Android) => panic!("Android is not supported"),
        None => {
            if cfg!(target_os = "linux") {
                linux::download_deps(enable_nvenc);
            } else {
                panic!("Unsupported platform");
            }
        }
    }
}

pub fn build_server_deps(platform: Option<BuildPlatform>, enable_nvenc: bool) {
    match platform {
        Some(BuildPlatform::Linux) => linux::build_deps(enable_nvenc),
        Some(BuildPlatform::Android) => panic!("Android is not supported"),
        None => {
            if cfg!(target_os = "linux") {
                linux::build_deps(enable_nvenc);
            } else {
                panic!("Unsupported platform");
            }
        }
    }
}
