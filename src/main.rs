use app::create_router;
use axum::{Router, routing::get, routing::post};
use config::config::Config;
use infra::db::setup_database;
use std::net::SocketAddr;
use tracing::info;

mod app;
mod app_state;
mod common;
mod config;
mod domains;
mod handlers;
mod infra;
mod middleware;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_env()?;
    let pool = setup_database(&config).await?;

    let state = app_state::AppState::new(&config, pool);

    let app = create_router(&config, state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Starting server on {addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;

    axum::serve(listener, app)
        .with_graceful_shutdown(app::shutdown_signal())
        .await?;

    info!("Server stopped");

    Ok(())
}
