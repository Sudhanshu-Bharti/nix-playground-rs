use log::LevelFilter;
use env_logger::Builder;
use std::str::FromStr;
// use env_logger::fmt::Color;
use owo_colors::OwoColorize;
use std::io::Write;

#[derive(Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Clone)]
pub struct Environment {
    log_level: LevelFilter,
    logger_name: String,
}

impl From<LogLevel> for LevelFilter {
    fn from(level: LogLevel) -> Self {
        match level {
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Warning => LevelFilter::Warn,
            LogLevel::Error => LevelFilter::Error,
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            log_level: LevelFilter::Info,
            logger_name: "nix_playground".to_string(),
        }
    }
}

impl Environment {
    pub fn init_logger(&self) {
        let mut builder = Builder::from_default_env();
        builder.filter_level(self.log_level).format(|buf, record| {
            let level = match record.level() {
                log::Level::Error => "ERROR".truecolor(255, 0, 0),
                log::Level::Warn  => "WARN".truecolor(255, 165, 0),
                log::Level::Info  => "INFO".truecolor(0, 200, 255),
                log::Level::Debug => "DEBUG".truecolor(180, 180, 180),
                log::Level::Trace => "TRACE".truecolor(150, 100, 255),
            };

            writeln!(buf, "[{}] {}", level, record.args())
        });
        builder.filter_module("reqwest", log::LevelFilter::Warn);
        builder.filter_module("hyper", log::LevelFilter::Warn);

        builder.init();
    }
}
impl Environment {
    pub fn with_log_level(level: LogLevel) -> Self {
        Self {
            log_level: level.into(),
            logger_name: "nix_playground".to_string(),
        }
    }
}
impl FromStr for LogLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" | "warning" => Ok(LogLevel::Warning),
            "error" => Ok(LogLevel::Error),
            _ => Err(format!("Invalid log level: {}", s)),
        }
    }
}