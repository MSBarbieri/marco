mod logger;
mod server;
mod settings;
mod tracer;
use log::SetLoggerError;
pub use logger::LogLevel;
pub use settings::Settings;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StartError {
    #[error("#[from] logger error")]
    LoggerInitError(#[from] SetLoggerError),
    #[error("Server creation Error '{0}")]
    InitServerError(#[from] server::ServerError),
    #[cfg(feature = "trace")]
    #[error("Server creation Error '{0}")]
    InitTracingError(#[from] tracer::TracingError),
}

pub async fn start_server(into_settings: impl Into<Settings>) -> Result<(), StartError> {
    let settings = into_settings.into();
    #[cfg(feature = "trace")]
    tracer::setup_tracing(&settings)?;

    // logger::start_logger(&cli.log_level)?;

    log::debug!("Cli Validated, starting server");
    server::create_server(settings).await?;
    Ok(())
}
