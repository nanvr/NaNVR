use std::{env, path::PathBuf};

use shared::{NANVR_HIGH_NAME, NANVR_LOW_NAME};

fn get_ffmpeg_build_path() -> PathBuf {
    filepaths::workspace_dir().join("thirdparty/ffmpeg/nanvr_build")
}

fn get_x264_build_path() -> PathBuf {
    filepaths::workspace_dir().join("thirdparty/x264/nanvr_build")
}

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let platform_subpath = "cpp/platform/linux";

    let common_iter = walkdir::WalkDir::new("cpp")
        .into_iter()
        .filter_entry(|entry| entry.file_name() != "tools" && entry.file_name() != "platform");

    let platform_iter = walkdir::WalkDir::new(platform_subpath).into_iter();

    let cpp_paths = common_iter
        .chain(platform_iter)
        .filter_map(|maybe_entry| maybe_entry.ok())
        .map(|entry| entry.into_path())
        .collect::<Vec<_>>();

    let source_files_paths = cpp_paths.iter().filter(|path| {
        path.extension()
            .filter(|ext| {
                let ext_str = ext.to_string_lossy();
                ext_str == "c" || ext_str == "cpp"
            })
            .is_some()
    });

    println!("cargo:rerun-if-changed=cpp");

    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++20")
        .flag_if_supported("-fdiagnostics-color=always")
        .files(source_files_paths)
        .include(filepaths::workspace_dir().join("thirdparty/openvr/headers"))
        .include("cpp");

    #[cfg(debug_assertions)]
    build.define(&format!("{NANVR_HIGH_NAME}_DEBUG_LOG"), None);

    let ffmpeg_path = get_ffmpeg_build_path();

    assert!(ffmpeg_path.join("include").exists());
    build.include(ffmpeg_path.join("include"));

    let x264_path = get_x264_build_path();

    assert!(x264_path.join("include").exists());
    build.include(x264_path.join("include"));

    build.define(&format!("{NANVR_HIGH_NAME}_GPL"), None);

    build.compile("bindings");

    let x264_path = get_x264_build_path();
    let x264_lib_path = x264_path.join("lib");

    println!(
        "cargo:rustc-link-search=native={}",
        x264_lib_path.to_string_lossy()
    );

    let x264_pkg_path = x264_lib_path.join("pkgconfig");
    assert!(x264_pkg_path.exists());

    let x264_pkg_path = x264_pkg_path.to_string_lossy().to_string();
    unsafe {
        env::set_var(
            "PKG_CONFIG_PATH",
            env::var("PKG_CONFIG_PATH").map_or(x264_pkg_path.clone(), |old| {
                format!("{x264_pkg_path}:{old}")
            }),
        )
    };
    println!("cargo:rustc-link-lib=static=x264");

    pkg_config::Config::new()
        .statik(true)
        .probe("x264")
        .unwrap();

    // ffmpeg
    let ffmpeg_path = get_ffmpeg_build_path();
    let ffmpeg_lib_path = ffmpeg_path.join("lib");

    assert!(ffmpeg_lib_path.exists());

    println!(
        "cargo:rustc-link-search=native={}",
        ffmpeg_lib_path.to_string_lossy()
    );

    let ffmpeg_pkg_path = ffmpeg_lib_path.join("pkgconfig");
    assert!(ffmpeg_pkg_path.exists());

    let ffmpeg_pkg_path = ffmpeg_pkg_path.to_string_lossy().to_string();
    unsafe {
        env::set_var(
            "PKG_CONFIG_PATH",
            env::var("PKG_CONFIG_PATH").map_or(ffmpeg_pkg_path.clone(), |old| {
                format!("{ffmpeg_pkg_path}:{old}")
            }),
        )
    };

    let pkg = pkg_config::Config::new().statik(true).to_owned();

    for lib in ["libavutil", "libavfilter", "libavcodec"] {
        pkg.probe(lib).unwrap();
    }

    bindgen::builder()
        .clang_arg("-xc++")
        .header(format!("cpp/{NANVR_LOW_NAME}_server/bindings.h"))
        .derive_default(true)
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("bindings.rs"))
        .unwrap();

    println!(
        "cargo:rustc-link-search=native={}",
        filepaths::workspace_dir()
            .join("thirdparty/openvr/lib/linux64")
            .to_string_lossy()
    );
    println!("cargo:rustc-link-lib=openvr_api");

    pkg_config::Config::new().probe("vulkan").unwrap();

    // fail build if there are undefined symbols in final library
    println!("cargo:rustc-cdylib-link-arg=-Wl,--no-undefined");
}
