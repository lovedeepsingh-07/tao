use crate::{schema, state};
use axum::response::IntoResponse;
use std::sync::Arc;
use tokio::sync::RwLock;

// GET (/api/report)
pub async fn route(
    axum::extract::State(server_state): axum::extract::State<Arc<RwLock<state::ServerState>>>,
) -> impl IntoResponse {
    let db_pool = {
        let state = server_state.read().await;
        state.db_pool.clone()
    };
    let reports: Vec<schema::Report> = sqlx::query_as::<_, schema::Report>("SELECT * FROM report")
        .fetch_all(&db_pool)
        .await
        .unwrap();

    axum::Json(reports)
}
