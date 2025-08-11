use crate::TargetBuildPlatform;

pub mod android;
pub mod linux;
pub enum OpenXRLoadersSelection {
    OnlyGeneric,
    OnlyPico,
    All,
}

pub fn prepare_server_deps(platform: Option<TargetBuildPlatform>, enable_nvenc: bool) {
    match platform {
        Some(TargetBuildPlatform::Linux) => linux::prepare_deps(enable_nvenc),
        Some(TargetBuildPlatform::Android) => panic!("Android is not supported"),
        None => linux::prepare_deps(enable_nvenc),
    }
}

pub fn download_server_deps(platform: Option<TargetBuildPlatform>, enable_nvenc: bool) {
    match platform {
        Some(TargetBuildPlatform::Linux) => linux::download_deps(enable_nvenc),
        Some(TargetBuildPlatform::Android) => panic!("Android is not supported"),
        None => linux::download_deps(enable_nvenc),
    }
}

pub fn build_server_deps(platform: Option<TargetBuildPlatform>, enable_nvenc: bool) {
    match platform {
        Some(TargetBuildPlatform::Linux) => linux::build_deps(enable_nvenc),
        Some(TargetBuildPlatform::Android) => panic!("Android is not supported"),
        None => linux::build_deps(enable_nvenc),
    }
}
