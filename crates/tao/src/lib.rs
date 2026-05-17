pub mod cli;
pub mod constants;
pub mod environment;
pub mod routes;
pub mod state;
pub mod schema;

use axum::routing;
pub use environment::Environment;
pub use state::ServerState;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors;

pub async fn run(db_url: &str, app_env: &str, server_port: u16) -> Result<(), error::Error> {
    let app_env = Environment::from(app_env);
    let server_state = ServerState::new(db_url, app_env.clone()).await?;

    let mut router = axum::Router::new()
        .route("/health", routing::get(routes::api::health))
        .fallback(routes::frontend::static_handler)
        .with_state(Arc::new(RwLock::new(server_state.clone())));

    if matches!(app_env, Environment::DEV) {
        router = router.layer(cors::CorsLayer::permissive());
    }

    let listener = tokio::net::TcpListener::bind((constants::SERVER_ADDRESS, server_port)).await?;
    log::info!(
        "tao server started on {}:{}",
        constants::SERVER_ADDRESS,
        server_port
    );
    axum::serve(listener, router).await?;

    Ok(())
}
