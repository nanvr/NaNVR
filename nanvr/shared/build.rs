use std::process::Command;

fn main() {
    let git_hash = get_git_hash();
    println!("cargo:rustc-env=BUILD_GIT_HASH={git_hash}");
}

fn get_git_hash() -> String {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let mut git_hash = String::from_utf8(output.stdout).unwrap();
    git_hash.truncate(8);
    git_hash
}
