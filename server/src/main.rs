use crate::settings::Settings;
use thiserror::Error;
use tracer::TracingError;

mod logger;
mod server;
mod settings;
mod tracer;

#[derive(Error, Debug)]
pub enum StartError {
    #[error("start server error '{0}")]
    InitServerError(#[from] server::ServerError),

    #[error("start tracer error '{0}'")]
    TracerError(#[from] TracingError),
}

#[cfg(feature = "trace")]
fn start_logging(settings: &Settings) -> Result<(), TracingError> {
    tracer::setup_tracing(&settings)?;
    Ok(())
}

#[cfg(not(feature = "trace"))]
fn start_logging(settings: &Settings) -> Result<(), SetLoggerError> {
    logger::start_logger(&settings.log_level)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), StartError> {
    let settings = Settings::default();
    start_logging(&settings)?;

    tracing::debug!("Cli Validated, starting server");
    server::create_server(settings).await?;
    Ok(())
}
