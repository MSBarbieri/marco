use clap::ValueEnum;
use log::SetLoggerError;
use serde::Deserialize;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, ValueEnum, Deserialize)]
pub enum LogLevel {
    #[default]
    Debug,
    Trace,
    Info,
    Wawn,
    Error,
}

impl From<LogLevel> for tracing::Level {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Trace => tracing::Level::TRACE,
            LogLevel::Debug => tracing::Level::DEBUG,
            LogLevel::Info => tracing::Level::INFO,
            LogLevel::Wawn => tracing::Level::WARN,
            LogLevel::Error => tracing::Level::ERROR,
        }
    }
}
impl ToString for LogLevel {
    fn to_string(&self) -> String {
        match self {
            LogLevel::Debug => "debug".to_string(),
            LogLevel::Trace => "trace".to_string(),
            LogLevel::Info => "info".to_string(),
            LogLevel::Wawn => "warn".to_string(),
            LogLevel::Error => "error".to_string(),
        }
    }
}

pub fn start_logger(log_level: &LogLevel) -> Result<(), SetLoggerError> {
    let log_level = log_level.to_string();
    std::env::set_var(
        "RUST_LOG",
        format!("{},axum_tracing_opentelemetry=info,otel=debug", log_level),
    );
    env_logger::try_init()?;
    Ok(())
}
