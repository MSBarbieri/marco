use axum::{response::IntoResponse, routing::get, Router};
use cfg_if::cfg_if;
use std::net::SocketAddr;
use thiserror::Error;

cfg_if! {
if #[cfg(feature = "trace")]{
use axum_tracing_opentelemetry::{opentelemetry_tracing_layer, response_with_trace_layer};
}}

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Port Used")]
    ConnnectionError,
    #[error("Database Not Found")]
    DatabaseNotFound,
    #[error("Cache Server not Found")]
    CacheDatabesNotFound,
    #[error("Server creation Error")]
    AxumError(#[from] hyper::Error),
    #[error("Unknown Start Server Error")]
    Unknown,
}
pub fn configure_routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
}
#[cfg(feature = "trace")]
pub fn set_layers(router: Router) -> Router {
    router
        .layer(response_with_trace_layer())
        .layer(opentelemetry_tracing_layer())
}

#[cfg(not(feature = "trace"))]
pub fn set_layer(router: Router) -> Router {
    router
}

pub async fn create_server(cli: crate::cli::Cli) -> Result<(), ServerError> {
    let addr: SocketAddr = cli.address.parse().unwrap();
    #[allow(unused_mut)] // this mut is used when is builded with telemetry enable.
    let mut router = configure_routes();
    router = set_layers(router);
    log::info!("Server Started with address: {:?}", cli.address.clone());
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}

async fn root() -> impl IntoResponse {
    "Hello, World!"
}

async fn health_check() -> impl IntoResponse {
    "OK"
}
