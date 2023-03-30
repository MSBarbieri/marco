use clap::Parser;
use server::cli::Cli;
#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    server::start_server(cli).await.unwrap();
}
