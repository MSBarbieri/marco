use clap::ValueEnum;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, ValueEnum)]

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
