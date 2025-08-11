use diffy::PatchFormatter;
use diffy::create_patch;
use filepaths as nanpaths;
use std::fs;
use std::mem;
use std::path::PathBuf;
use walkdir::WalkDir;
use xshell::{Shell, cmd};

fn files_to_format_paths() -> Vec<PathBuf> {
    let cpp_dir = nanpaths::crate_dir("server_openvr").join("cpp");

    WalkDir::new(cpp_dir)
        .into_iter()
        .filter_entry(|entry| {
            let included = entry.path().is_dir()
                || entry
                    .path()
                    .extension()
                    .is_some_and(|ext| matches!(ext.to_str().unwrap(), "c" | "cpp" | "h" | "hpp"));

            let excluded = matches!(
                entry.file_name().to_str().unwrap(),
                "shared" | "include" | "nvEncodeAPI.h"
            );

            included && !excluded
        })
        .filter_map(|entry| {
            let entry = entry.ok()?;
            entry.file_type().is_file().then(|| entry.path().to_owned())
        })
        .collect()
}

pub fn format() {
    let sh = Shell::new().unwrap();
    let dir = sh.push_dir(nanpaths::workspace_dir());

    cmd!(sh, "cargo fmt --all").run().unwrap();

    for path in files_to_format_paths() {
        cmd!(sh, "clang-format -i {path}").run().unwrap();
    }

    mem::drop(dir);
}

pub fn check_format() {
    let sh = Shell::new().unwrap();
    let dir = sh.push_dir(nanpaths::workspace_dir());

    cmd!(sh, "cargo fmt --all -- --check")
        .run()
        .expect("cargo fmt check failed");

    for path in files_to_format_paths() {
        let out_format = cmd!(sh, "clang-format -style=WebKit -dump-config")
            .read()
            .unwrap();
        eprintln!("\n{out_format}\n");
        let content = fs::read_to_string(&path).unwrap();
        let mut output = cmd!(sh, "clang-format {path}").read().unwrap();

        if !content.ends_with('\n') {
            panic!("file {} missing final newline", path.display());
        }
        output.push('\n');

        if content != output {
            let diff_out = create_patch(&content, &output);
            let formatter = PatchFormatter::new().with_color();
            panic!(
                "clang-format check failed for {}, diff: {}",
                path.display(),
                formatter.fmt_patch(&diff_out)
            );
        }
    }

    mem::drop(dir);
}
