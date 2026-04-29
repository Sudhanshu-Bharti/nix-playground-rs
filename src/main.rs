pub mod constants;
pub mod environment;
pub mod cli;
pub mod patch;
pub mod clean;

use clap::Parser;
use cli::{Cli, Commands};
use environment::{Environment, LogLevel};

fn main() {
    let cli = Cli::parse();
    let log_level: LogLevel = cli
        .log_level
        .unwrap_or_else(|| "info".to_string())
        .parse()
        .unwrap_or(LogLevel::Info);
    match cli.command {
        Commands::Patch => {
            patch::create_diff().expect("failed to create diff");
        }
    }
    let env = Environment::with_log_level(log_level);
    env.init_logger();

    log::info!("Starting nix-playground");
}
