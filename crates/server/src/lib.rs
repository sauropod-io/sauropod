//! The Sauropod Scales server.

mod observability;
mod server;

pub use server::Server;

use tower_http::cors::{Any, CorsLayer};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Axum error: {0}")]
    Axum(#[from] axum::Error),
}

pub async fn run_server(
    config: &sauropod_config::Config,
    server: std::sync::Arc<server::Server>,
) -> Result<(), Error> {
    let listener = tokio::net::TcpListener::bind((
        config.host.as_deref().unwrap_or("::"),
        config.port.unwrap_or(3140),
    ))
    .await?;
    let cors = CorsLayer::new().allow_origin(Any);
    let api = sauropod_http::register_routes(server).layer(cors);
    let app = sauropod_http::make_ui_routes().nest(sauropod_http::API_PREFIX, api);

    tracing::info!("listening on {}", listener.local_addr().unwrap());
    Ok(axum::serve(listener, app).await?)
}
