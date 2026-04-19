pub mod cli;
pub mod constants;
pub mod environment;
pub mod routes;
pub mod state;

use axum::routing;
pub use environment::Environment;
pub use state::ServerState;
use std::sync::Arc;
use tokio::sync::RwLock;

pub async fn run(db_url: &str, app_env: &str, server_port: u16) -> Result<(), error::Error> {
    let server_state = Arc::new(RwLock::new(
        ServerState::new(db_url, Environment::from(app_env)).await?,
    ));

    let router = axum::Router::new()
        .route("/health", routing::get(routes::health))
        .fallback(routes::frontend::static_handler)
        .with_state(server_state);

    let listener = tokio::net::TcpListener::bind((constants::SERVER_ADDRESS, server_port)).await?;
    log::info!(
        "tao server started on {}:{}",
        constants::SERVER_ADDRESS,
        server_port
    );
    axum::serve(listener, router).await?;

    Ok(())
}
