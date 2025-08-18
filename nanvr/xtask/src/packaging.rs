use crate::{
    build::{self, Profile},
    command,
    dependencies::{self, OpenXRLoadersSelection},
};

use clap::ValueEnum;
use shared::NANVR_HIGH_NAME;
use shared::NANVR_LOW_NAME;
use std::{fs, path::Path};
use xshell::{Shell, cmd};

#[derive(Clone, Copy, Default, ValueEnum)]
pub enum ReleaseFlavor {
    #[default]
    GitHub,
    MetaStore,
    PicoStore,
}

pub fn include_licenses(root_path: &Path) {
    let sh = Shell::new().unwrap();

    // Add licenses
    let licenses_dir = root_path.join("licenses");
    sh.create_dir(&licenses_dir).unwrap();
    sh.copy_file(
        filepaths::workspace_dir().join("LICENSE"),
        licenses_dir.join(format!("{NANVR_HIGH_NAME}.txt")),
    )
    .unwrap();
    sh.copy_file(
        filepaths::crate_dir("server_openvr").join("LICENSE-Valve"),
        licenses_dir.join("Valve.txt"),
    )
    .unwrap();

    // Gather licenses with cargo about
    cmd!(sh, "cargo install cargo-about --version 0.6.4")
        .run()
        .unwrap();
    let licenses_template = filepaths::crate_dir("xtask").join("licenses_template.hbs");
    let licenses_content = cmd!(sh, "cargo about generate {licenses_template}")
        .read()
        .unwrap();
    sh.write_file(licenses_dir.join("dependencies.html"), licenses_content)
        .unwrap();
}

pub fn package_streamer(enable_nvenc: bool, root: Option<String>) {
    let sh = Shell::new().unwrap();

    dependencies::linux::clean_and_build_server_deps(enable_nvenc);

    build::build_streamer(
        Profile::Distribution,
        root,
        crate::CommonBuildFlags {
            locked: true,
            ..Default::default()
        },
        false,
    );

    include_licenses(&filepaths::streamer_build_dir());

    command::targz(&sh, &filepaths::streamer_build_dir()).unwrap();
}

pub fn package_launcher() {
    let sh = Shell::new().unwrap();

    sh.remove_path(filepaths::launcher_build_dir()).ok();

    build::build_launcher(
        Profile::Distribution,
        &crate::CommonBuildFlags {
            locked: true,
            ..Default::default()
        },
    );

    include_licenses(&filepaths::launcher_build_dir());

    command::targz(&sh, &filepaths::launcher_build_dir()).unwrap();
}

pub fn replace_client_openxr_manifest(from_pattern: &str, to: &str) {
    let manifest_path = filepaths::crate_dir("client_openxr").join("Cargo.toml");
    let manifest_string = fs::read_to_string(&manifest_path)
        .unwrap()
        .replace(from_pattern, to);

    fs::write(manifest_path, manifest_string).unwrap();
}

pub fn package_client_openxr(flavor: ReleaseFlavor) {
    fs::remove_dir_all(filepaths::deps_dir().join("android_openxr")).ok();

    let openxr_selection = match flavor {
        ReleaseFlavor::GitHub => OpenXRLoadersSelection::All,
        ReleaseFlavor::MetaStore => OpenXRLoadersSelection::OnlyGeneric,
        ReleaseFlavor::PicoStore => OpenXRLoadersSelection::OnlyPico,
    };

    dependencies::android::build_deps(false, &openxr_selection);

    if !matches!(flavor, ReleaseFlavor::GitHub) {
        replace_client_openxr_manifest(
            &format!(r#"package = "{NANVR_LOW_NAME}.client.stable""#),
            &format!(r#"package = "{NANVR_LOW_NAME}.client""#),
        );
    }

    if matches!(flavor, ReleaseFlavor::MetaStore) {
        replace_client_openxr_manifest(r#"value = "all""#, r#"value = "quest2|questpro|quest3""#);
    }

    build::build_android_client(Profile::Distribution);
}

pub fn package_client_lib(link_stdcpp: bool, all_targets: bool) {
    let sh = Shell::new().unwrap();

    build::build_android_client_core_lib(Profile::Distribution, link_stdcpp, all_targets);

    command::zip(
        &sh,
        &filepaths::build_dir().join(format!("{NANVR_LOW_NAME}_client_core")),
    )
    .unwrap();
}
