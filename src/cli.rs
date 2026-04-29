use clap::Parser;
use std::env;
use std::str::FromStr;
use crate::environment::{Environment, LogLevel};
#[derive(Parser,Debug)]
#[command(
    name= "nix-playground",
    about = "xx",
    version = "0.0.1"
)]
pub struct Cli {
    #[arg(short, long)]
    pub log_level: Option<String>,
}
#[derive(Subcommand,Debug)]
pub enum Commands {
    /// Output the patch file to apply on the current nix package
    Patch,
}
impl Cli {
    pub fn init_env(self) -> Environment {
        let log_level_str = self.log_level
            .or_else(|| env::var("RUST_LOG").ok())
            .unwrap_or_else(|| "info".to_string());
        let log_level = LogLevel::from_str(&log_level_str).unwrap_or_else(|_| {
            eprintln!("Invalid log level provided '{}'. Using INFO log level.", log_level_str);
            LogLevel::Info
        });
        let env = Environment::with_log_level(log_level);
        env.init_logger();

        env
    }
}








