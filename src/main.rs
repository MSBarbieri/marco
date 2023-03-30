use clap::Parser;
use server::ServerError;
use thiserror::Error;
use tracer::TracingError;

use crate::cli::{Cli, CliError};

mod cli;
mod logger;
mod server;
mod tracer;

#[derive(Error, Debug)]
pub enum StartError {
    #[error("CliError Invalid with error: '{0}")]
    CliError(#[from] CliError),
    #[error("start server error '{0}")]
    ServerError(#[from] ServerError),
    #[error("start tracer error '{0}'")]
    TracerError(#[from] TracingError),
}

#[tokio::main]
async fn main() -> Result<(), StartError> {
    let cli = Cli::parse();
    cli.validate()?;
    tracer::setup_tracing(&cli)?;

    tracing::debug!("Cli Validated, starting server");
    server::create_server(cli).await?;
    Ok(())
}
