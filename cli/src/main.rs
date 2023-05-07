use clap::Parser;
use server::Settings;

#[derive(Parser)]
struct StartServer {
    #[clap(short, long)]
    address: String,
    #[clap(short, long)]
    port: u16,
}

impl Into<Settings> for StartServer {
    fn into(self) -> Settings {
        Settings::new(None, self.address, server::LogLevel::Debug)
    }
}

#[tokio::main]
async fn main() {
    let cli = StartServer::parse();
    server::start_server(cli).await.unwrap();
}
