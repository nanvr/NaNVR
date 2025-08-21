use clap::{Args, Parser, Subcommand, ValueEnum};
use shared::NANVR_WEBSERVER_PORT;

#[derive(Parser)]
#[command(name = "nanvr_cli")]
#[command(about = "CLI for NaNVR", long_about = None)]
struct Cli {
    /// Web server port to communicate with driver.
    #[arg(long, default_value_t = NANVR_WEBSERVER_PORT)]
    web_server_port: u16,
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set value in configuration and apply it to server
    SetValue {
        /// Path to the configuration value
        #[arg(long)]
        path: String,
    },
    /// Get value from configuration
    GetValue {
        /// Path to the configuration value
        #[arg(long)]
        path: String,
    },
    StartSteamvr,
}

fn set_value(path: String) {}
fn get_value(path: String) {}

fn main() {
    let cli = Cli::parse();
    match cli.commands {
        Commands::SetValue { path } => set_value(path),
        Commands::GetValue { path } => get_value(path),
        Commands::StartSteamvr => todo!(),
    }
}
