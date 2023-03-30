use crate::logger::LogLevel;
use clap::Parser;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    #[error("Limit of threads exeeded: '{0}', limit is 8")]
    ThreadLimit(usize),
    #[error("Invalid type of address '{0}'")]
    InvaidAddress(String),
}

#[derive(Debug, Clone, Default, Parser)]
#[command(author,version,about,long_about = None)]
pub struct Cli {
    ///async treadpool size
    #[arg(short, long, default_value_t = 8)]
    pub num_threads: usize,

    /// Log/Tracing level
    #[arg(value_enum, short, long, default_value_t = LogLevel::Debug)]
    pub log_level: LogLevel,

    /// server address
    #[arg(short, long, default_value_t = String::from("127.0.0.1:3000"))]
    pub address: String,

    #[cfg(feature = "trace")]
    #[arg(short, long, default_value_t = String::from("http://localhost:4317"))]
    pub otlp_url: String,
}

impl Cli {
    pub fn validate(&self) -> Result<(), CliError> {
        if self.num_threads > 8 {
            return Err(CliError::ThreadLimit(self.num_threads));
        }

        if self.address.parse::<std::net::SocketAddr>().is_err() {
            return Err(CliError::InvaidAddress(self.address.clone()));
        }
        Ok(())
    }
}
