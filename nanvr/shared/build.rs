use std::process::Command;

fn main() {
    let build_id = if let Ok(release) = std::env::var("NAMED_RELEASE") {
        release
    } else {
        get_git_hash()
    };
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rustc-env=BUILD_ID={build_id}");
}

fn get_git_hash() -> String {
    let output = Command::new("git")
        .args(["rev-parse", "--short=8", "HEAD"])
        .output()
        .unwrap();
    String::from_utf8(output.stdout).unwrap()
}
