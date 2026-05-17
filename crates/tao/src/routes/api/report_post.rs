use crate::{schema, state};
use axum::response::IntoResponse;
use std::sync::Arc;
use tokio::sync::RwLock;

// POST (/api/report)
pub async fn route(
    axum::extract::State(server_state): axum::extract::State<Arc<RwLock<state::ServerState>>>,
    axum::extract::Json(report): axum::extract::Json<schema::Report>,
) -> impl IntoResponse {
    let db_pool = {
        let state = server_state.read().await;
        state.db_pool.clone()
    };
    sqlx::query("INSERT INTO report (body, kind) VALUES ($1, $2)")
        .bind(&report.body)
        .bind(&report.kind)
        .execute(&db_pool)
        .await
        .unwrap();
    ().into_response()
}
