pub mod cli;
pub mod constants;
pub mod environment;
pub mod routes;
pub mod schema;
pub mod state;

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
        .route("/api/health", routing::get(routes::api::health))
        .route(
            "/api/project",
            routing::get(routes::api::project_get_all).post(routes::api::project_post),
        )
        .route(
            "/api/project/{project_id}",
            routing::get(routes::api::project_get_one).delete(routes::api::project_delete),
        )
        .route(
            "/api/project/{project_id}/report",
            routing::get(routes::api::report_get).post(routes::api::report_post),
        )
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
