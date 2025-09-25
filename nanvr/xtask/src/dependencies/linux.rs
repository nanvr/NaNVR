use xshell::{Shell, cmd};

pub fn clean_and_build_server_deps(enable_nvenc: bool) {
    clean_deps();

    build_x264();
    build_ffmpeg(enable_nvenc);
}

fn clean_deps() {
    let sh = Shell::new().unwrap();
    sh.change_dir(filepaths::workspace_dir());

    // Clean submodule folders from previous build directories and patches
    let ffmpeg_command = "for p in thirdparty/*; do (cd $p; git reset --hard; git clean -df); done";
    cmd!(sh, "bash -c {ffmpeg_command}").run().unwrap();
}

fn x264_path() -> std::path::PathBuf {
    filepaths::workspace_dir().join("thirdparty/x264")
}

fn ffmpeg_path() -> std::path::PathBuf {
    filepaths::workspace_dir().join("thirdparty/ffmpeg")
}

fn nvenc_headers_path() -> std::path::PathBuf {
    filepaths::workspace_dir().join("thirdparty/nv-codec-headers")
}

fn build_x264() {
    let sh = Shell::new().unwrap();

    let x264_src_path = x264_path();

    let flags = ["--enable-static", "--disable-cli", "--enable-pic"];

    let build_path = x264_src_path.join("nanvr_build");
    sh.remove_path(&build_path).ok();

    let install_prefix = format!("--prefix={}", build_path.display());

    let _push_guard = sh.push_dir(x264_src_path);

    cmd!(sh, "./configure {install_prefix} {flags...}")
        .run()
        .unwrap();

    let nproc = cmd!(sh, "nproc").read().unwrap();
    cmd!(sh, "make -j{nproc}").run().unwrap();
    cmd!(sh, "make install").run().unwrap();
}

fn build_ffmpeg(enable_nvenc: bool) {
    let ffmpeg_src_path = ffmpeg_path();

    let sh = Shell::new().unwrap();

    let flags = [
        "--enable-gpl",
        "--enable-version3",
        "--enable-static",
        "--disable-programs",
        "--disable-doc",
        "--disable-avdevice",
        "--disable-avformat",
        "--disable-swresample",
        "--disable-swscale",
        "--disable-postproc",
        "--disable-network",
        "--disable-everything",
        "--enable-encoder=h264_vaapi",
        "--enable-encoder=hevc_vaapi",
        "--enable-encoder=av1_vaapi",
        "--enable-hwaccel=h264_vaapi",
        "--enable-hwaccel=hevc_vaapi",
        "--enable-hwaccel=av1_vaapi",
        "--enable-filter=scale_vaapi",
        "--enable-vulkan",
        "--enable-libdrm",
        "--enable-pic",
        "--enable-rpath",
        "--fatal-warnings",
    ];

    let build_path = ffmpeg_src_path.join("nanvr_build");
    sh.remove_path(&build_path).ok();

    let install_prefix = format!("--prefix={}", build_path.display());
    // The reason for 4x$ in LDSOFLAGS var refer to https://stackoverflow.com/a/71429999
    // all varients of --extra-ldsoflags='-Wl,-rpath,$ORIGIN' do not work! don't waste your time trying!
    //
    let config_vars = r"-Wl,-rpath,'$$$$ORIGIN'";

    let _push_guard = sh.push_dir(ffmpeg_src_path);
    let _env_vars = sh.push_env("LDSOFLAGS", config_vars);

    // Patches ffmpeg for workarounds and patches that have yet to be unstreamed
    let ffmpeg_command = "for p in ../../nanvr/xtask/patches/*; do patch -p1 < $p; done";
    cmd!(sh, "bash -c {ffmpeg_command}").run().unwrap();

    if enable_nvenc {
        /*
           Describing Nvidia specific options --nvccflags:
           nvcc from CUDA toolkit version 11.0 or higher does not support compiling for 'compute_30' (default in ffmpeg)
           52 is the minimum required for the current CUDA 11 version (Quadro M6000 , GeForce 900, GTX-970, GTX-980, GTX Titan X)
           https://arnon.dk/matching-sm-architectures-arch-and-gencode-for-various-nvidia-cards/
           Anyway below 50 arch card don't support nvenc encoding hevc https://developer.nvidia.com/nvidia-video-codec-sdk (Supported devices)
           Nvidia docs:
           https://docs.nvidia.com/video-technologies/video-codec-sdk/ffmpeg-with-nvidia-gpu/#commonly-faced-issues-and-tips-to-resolve-them
        */
        let nvenc_headers_path = nvenc_headers_path();
        let header_build_dir = nvenc_headers_path.join("nanvr_build");
        sh.remove_path(&header_build_dir).ok();
        {
            let make_header_cmd = format!("make install PREFIX='{}'", header_build_dir.display());
            let _header_push_guard = sh.push_dir(nvenc_headers_path);
            cmd!(sh, "bash -c {make_header_cmd}").run().unwrap();
        }

        let cuda = pkg_config::Config::new().probe("cuda").unwrap();
        let include_flags = cuda
            .include_paths
            .iter()
            .map(|path| format!("-I{}", path.to_string_lossy()))
            .reduce(|a, b| format!("{a} {b}"))
            .expect("pkg-config cuda entry to have include-paths");
        let link_flags = cuda
            .link_paths
            .iter()
            .map(|path| format!("-L{}", path.to_string_lossy()))
            .reduce(|a, b| format!("{a} {b}"))
            .expect("pkg-config cuda entry to have link-paths");

        let nvenc_flags = &[
            "--enable-encoder=h264_nvenc",
            "--enable-encoder=hevc_nvenc",
            "--enable-encoder=av1_nvenc",
            "--enable-nonfree",
            "--enable-cuda-nvcc",
            "--enable-libnpp",
            "--nvccflags=\"-gencode arch=compute_52,code=sm_52 -O2\"",
            &format!("--extra-cflags=\"{include_flags}\""),
            &format!("--extra-ldflags=\"{link_flags}\""),
        ];

        let env_vars = format!(
            "PKG_CONFIG_PATH='{}'",
            header_build_dir.join("lib/pkgconfig").display()
        );
        let flags_combined = flags.join(" ");
        let nvenc_flags_combined = nvenc_flags.join(" ");

        let command = format!(
            "{env_vars} ./configure {install_prefix} {flags_combined} {nvenc_flags_combined}"
        );

        cmd!(sh, "bash -c {command}").run().unwrap();
    } else {
        cmd!(sh, "./configure {install_prefix} {flags...}")
            .run()
            .unwrap();
    }

    let nproc = cmd!(sh, "nproc").read().unwrap();
    cmd!(sh, "make -j{nproc}").run().unwrap();
    cmd!(sh, "make install").run().unwrap();
}
