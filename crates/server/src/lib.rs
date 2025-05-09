//! The Sauropod server.

mod cli;
mod observability;
mod server;
pub use cli::ClapConfigSource;
pub use cli::generated::{add_config_flags, clap_to_config_source};
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
    let listener =
        tokio::net::TcpListener::bind((config.host.as_deref().unwrap_or("::"), config.port))
            .await?;
    let cors = CorsLayer::new().allow_origin(Any);
    let api = sauropod_http::register_routes(server)
        .layer(cors)
        .layer(axum::extract::DefaultBodyLimit::max(100_000_000));
    let app = sauropod_http::make_ui_routes().nest(sauropod_http::API_PREFIX, api);

    tracing::info!("Listening on {}", listener.local_addr().unwrap());
    Ok(axum::serve(listener, app).await?)
}
