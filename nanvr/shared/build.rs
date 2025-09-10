use std::process::Command;

use vergen_git2::{Emitter, Git2Builder};

fn main() {
    let build_id = if let Some(release) = option_env!("NAMED_RELEASE") {
        if release.is_empty() {
            panic!("Release variable was defined, but is empty!")
        }
        release
    } else {
        &get_git_hash()
    };
    println!("cargo:rustc-env=BUILD_ID={build_id}");

    let git2 = Git2Builder::default()
        .commit_timestamp(true) // todo: might not be required
        //                                           also VERGEN_GIT_SHA contains commit sha
        .build()
        .unwrap();
    Emitter::default()
        .add_instructions(&git2)
        .unwrap()
        .emit()
        .unwrap();
}

fn get_git_hash() -> String {
    let output = Command::new("git")
        .args(["rev-parse", "--short=8", "HEAD"])
        .output()
        .unwrap();
    String::from_utf8(output.stdout).unwrap()
}
