use clap::Parser;
use thiserror::Error;
use tracer::TracingError;

use crate::cli::Cli;

mod cli;
mod logger;
mod server;
mod tracer;

#[derive(Error, Debug)]
pub enum StartError {
    #[error("CliError Invalid with error: '{0}")]
    CliError(#[from] cli::CliError),
    #[error("start server error '{0}")]
    InitServerError(#[from] server::ServerError),

    #[error("start tracer error '{0}'")]
    TracerError(#[from] TracingError),
}
#[cfg(feature = "trace")]
fn start_logging(cli: &Cli) -> Result<(), TracingError> {
    tracer::setup_tracing(&cli)?;
    Ok(())
}

#[cfg(not(feature = "trace"))]
fn start_logging(cli: &Cli) -> Result<(), SetLoggerError> {
    logger::start_logger(&cli.log_level)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), StartError> {
    let cli = Cli::parse();
    cli.validate()?;
    start_logging(&cli)?;

    tracing::debug!("Cli Validated, starting server");
    server::create_server(cli).await?;
    Ok(())
}
