mod build;
mod ci;
mod command;
mod dependencies;
mod format;
mod packaging;
mod version;

use crate::build::Profile;
use clap::{Args, Parser, Subcommand, ValueEnum};
use dependencies::OpenXRLoadersSelection;

use filepaths::Layout;
use packaging::ReleaseFlavor;
use std::{fs, time::Instant};
use xshell::{Shell, cmd};

#[derive(Parser)]
#[command(name = "cargo xtask")]
#[command(about = "Developement actions for NaNVR", long_about = None)]
struct Cli {
    #[command(subcommand)]
    commands: Commands,
}
#[derive(Subcommand)]
enum Commands {
    /// Download and compile streamer or/and client external dependencies
    BuildDeps {
        /// If not specified, prepares server and android dependencies at the same time
        #[arg(long, value_enum)]
        platform: Option<TargetBuildPlatform>,
        /// Build for all android supported ABI targets
        #[arg(long)]
        all_targets: bool,
        /// Enables nvenc support on Linux
        #[arg(long)]
        enable_nvenc: bool,
    },
    /// Compile streamer external dependencies
    BuildServerDeps {
        /// Enables nvenc support on Linux
        #[arg(long)]
        enable_nvenc: bool,
    },
    /// Build streamer, then copy binaries to build folder
    BuildStreamer {
        /// Preserve the configuration file between rebuilds (session.json)
        #[arg(long)]
        keep_config: bool,
        #[arg(long, value_enum, default_value_t = Profile::Debug)]
        profile: Profile,
        #[clap(flatten)]
        common_build_flags: CommonBuildFlags,
    },
    /// Build launcher, then copy binaries to build folder
    BuildLauncher {
        #[arg(long, value_enum, default_value_t = Profile::Debug)]
        profile: Profile,
        #[clap(flatten)]
        common_build_flags: CommonBuildFlags,
    },
    /// Build a C-ABI NaNVR server library and header
    BuildServerLib {
        #[arg(long, value_enum, default_value_t = Profile::Debug)]
        profile: Profile,
        #[clap(flatten)]
        common_build_flags: CommonBuildFlags,
    },
    /// Build client, then copy binaries to build folder.
    /// Requires `JAVA_HOME` set to at least JDK17 folder,
    /// `ANDROID_NDK_HOME` set to Android NDK 25.1.8937893 folder,
    /// `ANDROID_HOME` set to Android SDK folder
    BuildClient {
        #[arg(long, value_enum, default_value_t = Profile::Debug)]
        profile: Profile,
    },
    /// Build a C-ABI NaNVR client library and header
    BuildClientLib {
        #[arg(long, value_enum, default_value_t = Profile::Debug)]
        profile: Profile,
        /// Configure linking to libc++_shared with build-client-lib
        #[arg(long, default_value_t = true)]
        link_stdcpp: bool,
        /// Build for all android supported ABI targets
        #[arg(long)]
        all_targets: bool,
    },
    /// Build a C-ABI NaNVR OpenXR entry point client library and header
    BuildClientXrLib {
        #[arg(long, value_enum, default_value_t = Profile::Debug)]
        profile: Profile,
        /// Configure linking to libc++_shared with build-client-lib
        #[arg(long, default_value_t = true)]
        link_stdcpp: bool,
    },
    /// Build streamer and then open the dashboard
    RunStreamer {
        #[arg(long, value_enum, default_value_t = Profile::Debug)]
        profile: Profile,
        #[clap(flatten)]
        common_build_flags: CommonBuildFlags,
        /// Preserve the configuration file between rebuilds (session.json)
        #[arg(long)]
        keep_config: bool,
    },
    /// Build launcher and then open it
    RunLauncher {
        #[arg(long, value_enum, default_value_t = Profile::Debug)]
        profile: Profile,
        #[clap(flatten)]
        common_build_flags: CommonBuildFlags,
    },
    /// Build streamer with distribution profile, make archive
    PackageStreamer {
        /// Installation root. By default no root is set and paths are calculated
        /// using relative paths, which requires conforming to FHS on Linux
        #[arg(long)]
        root: Option<String>,
        /// Enables nvenc support on Linux
        #[arg(long)]
        enable_nvenc: bool,
    },
    /// Build launcher with distribution profile, make archive
    PackageLauncher,
    /// Build client with distribution profile
    PackageClient {
        #[arg(long, value_enum, default_value_t = ReleaseFlavor::GitHub)]
        package_flavor: ReleaseFlavor,
    },
    /// Build client library then zip it
    PackageClientLib {
        /// Configure linking to libc++_shared with build-client-lib
        #[arg(long, default_value_t = true)]
        link_stdcpp: bool,
        /// Build for all android supported ABI targets
        #[arg(long)]
        all_targets: bool,
    },
    /// Autoformat the code
    Format {
        /// Only check if code is correctly formatted
        #[arg(long)]
        check: bool,
    },
    /// Removes all build artifacts and dependencies
    Clean,
    /// Bump streamer and client package versions
    Bump {
        /// Bump version
        #[arg(long)]
        version: Option<String>,
        /// Append nightly tag to versions
        #[arg(long)]
        is_nightly: bool,
    },
    /// Show warnings for selected clippy lints
    CiClippy,
    /// Verify MSRV version
    CheckMsrv,
}

#[derive(Clone, ValueEnum)]
enum TargetBuildPlatform {
    Linux,
    Android,
}

#[derive(Default, Clone, Args)]
pub struct CommonBuildFlags {
    /// Forces build subcommands to use only dependencies specified from Cargo.lock
    #[arg(long)]
    locked: bool,
    /// Forces build subcommands to use locally cached dependencies specified in Cargo.lock
    /// and fail if internet access was required during build
    #[arg(long)]
    frozen: bool,
    /// Forces build subcommands to fail if they try to use internet.
    /// Note that 'xtask' and 'cargo about' dependencies are downloaded and built during build of nanvr
    #[arg(long)]
    offline: bool,
    /// Enable Profiling
    #[arg(long)]
    profiling: bool,
}

pub fn run_streamer() {
    let sh = Shell::new().unwrap();

    let dashboard_exe = Layout::new(&filepaths::streamer_build_dir()).dashboard_exe();
    cmd!(sh, "{dashboard_exe}").run().unwrap();
}

pub fn run_launcher() {
    let sh = Shell::new().unwrap();

    let launcher_exe = filepaths::launcher_build_exe_path();
    cmd!(sh, "{launcher_exe}").run().unwrap();
}

pub fn clean() {
    fs::remove_dir_all(filepaths::build_dir()).ok();
    fs::remove_dir_all(filepaths::deps_dir()).ok();
    if filepaths::target_dir() == filepaths::workspace_dir().join("target") {
        // Detete target folder only if in the local wokspace!
        fs::remove_dir_all(filepaths::target_dir()).ok();
    }
}

fn main() {
    let cli = Cli::parse();

    let begin_time = Instant::now();
    match cli.commands {
        Commands::BuildDeps {
            platform,
            all_targets,
            enable_nvenc,
        } => {
            if let Some(platform) = platform {
                if matches!(platform, TargetBuildPlatform::Android) {
                    dependencies::android::build_deps(all_targets, &OpenXRLoadersSelection::All);
                } else {
                    dependencies::linux::build_server_deps(enable_nvenc);
                }
            } else {
                dependencies::linux::build_server_deps(enable_nvenc);

                dependencies::android::build_deps(all_targets, &OpenXRLoadersSelection::All);
            }
        }
        Commands::BuildServerDeps { enable_nvenc } => {
            dependencies::linux::build_server_deps(enable_nvenc);
        }
        Commands::BuildStreamer {
            keep_config,
            profile,
            common_build_flags,
        } => build::build_streamer(profile, None, common_build_flags, keep_config),
        Commands::BuildLauncher {
            profile,
            common_build_flags,
        } => build::build_launcher(profile, &common_build_flags),
        Commands::BuildServerLib {
            profile,
            common_build_flags,
        } => build::build_server_lib(profile, None, common_build_flags),
        Commands::BuildClient { profile } => build::build_android_client(profile),
        Commands::BuildClientLib {
            profile,
            link_stdcpp,
            all_targets,
        } => build::build_android_client_core_lib(profile, link_stdcpp, all_targets),
        Commands::BuildClientXrLib {
            profile,
            link_stdcpp,
        } => build::build_android_client_openxr_lib(profile, link_stdcpp),
        Commands::RunStreamer {
            profile,
            common_build_flags,
            keep_config,
        } => {
            build::build_streamer(profile, None, common_build_flags, keep_config);
            run_streamer();
        }
        Commands::RunLauncher {
            profile,
            common_build_flags,
        } => {
            build::build_launcher(profile, &common_build_flags);
            run_launcher();
        }
        Commands::PackageStreamer { root, enable_nvenc } => {
            packaging::package_streamer(enable_nvenc, root);
        }
        Commands::PackageLauncher => packaging::package_launcher(),
        Commands::PackageClient { package_flavor } => {
            packaging::package_client_openxr(package_flavor);
        }
        Commands::PackageClientLib {
            link_stdcpp,
            all_targets,
        } => packaging::package_client_lib(link_stdcpp, all_targets),
        Commands::Format { check } => {
            if check {
                format::check_format();
            } else {
                format::format()
            }
        }
        Commands::Clean => clean(),
        Commands::Bump {
            version,
            is_nightly,
        } => version::bump_version(version, is_nightly),
        Commands::CiClippy => ci::clippy_ci(),
        Commands::CheckMsrv => version::check_msrv(),
    }
    let elapsed_time = begin_time.elapsed();

    // todo: useless on run commands
    println!(
        "\nDone [{}m {}s]\n",
        elapsed_time.as_secs() / 60,
        elapsed_time.as_secs() % 60
    );
}
