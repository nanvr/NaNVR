use shared::NANVR_LOW_NAME;

fn main() {
    use std::{env, path::PathBuf};
    use xshell::{Shell, cmd};

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let target_dir = out_dir.join("../../..");

    let sh = Shell::new().unwrap();
    let command = format!(
        "g++ -shared -fPIC $(pkg-config --cflags libdrm) drm-lease-shim.cpp -o {}/{NANVR_LOW_NAME}_drm_lease_shim.so",
        target_dir.display()
    );
    cmd!(sh, "bash -c {command}").run().unwrap();
}
