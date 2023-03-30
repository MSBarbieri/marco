pub mod cli;
mod logger;
mod server;
use cli::Cli;
use log::SetLoggerError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StartError {
    #[error("#[from] logger error")]
    LoggerInitError(#[from] SetLoggerError),
    #[error("Server creation Error '{0}")]
    InitServerError(#[from] server::ServerError),
    #[error("#[from] cli")]
    CliError(#[from] cli::CliError),
}

pub async fn start_server(into_cli: impl Into<Cli>) -> Result<(), StartError> {
    let cli: Cli = into_cli.into();
    cli.validate()?;
    logger::start_logger(&cli.log_level)?;

    log::debug!("Cli Validated, starting server");
    server::create_server(cli).await?;
    Ok(())
}
