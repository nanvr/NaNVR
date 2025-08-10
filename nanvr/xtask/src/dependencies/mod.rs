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
        None => linux::prepare_deps(enable_nvenc),
    }
}

pub fn download_server_deps(platform: Option<BuildPlatform>, enable_nvenc: bool) {
    match platform {
        Some(BuildPlatform::Linux) => linux::download_deps(enable_nvenc),
        Some(BuildPlatform::Android) => panic!("Android is not supported"),
        None => linux::download_deps(enable_nvenc),
    }
}

pub fn build_server_deps(platform: Option<BuildPlatform>, enable_nvenc: bool) {
    match platform {
        Some(BuildPlatform::Linux) => linux::build_deps(enable_nvenc),
        Some(BuildPlatform::Android) => panic!("Android is not supported"),
        None => linux::build_deps(enable_nvenc),
    }
}
