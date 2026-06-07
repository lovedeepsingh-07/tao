use crate::{schema, state};
use axum::response::IntoResponse;
use std::sync::Arc;
use tokio::sync::RwLock;

// GET (/api/project/{project_id}/report)
pub async fn get(
    axum::extract::State(server_state): axum::extract::State<Arc<RwLock<state::ServerState>>>,
    axum::extract::Path(project_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let db_pool = {
        let state = server_state.read().await;
        state.db_pool.clone()
    };
    let reports: Vec<schema::Report> =
        sqlx::query_as::<_, schema::Report>("SELECT * FROM report WHERE project_id = $1")
            .bind(&uuid::Uuid::parse_str(&project_id).unwrap())
            .fetch_all(&db_pool)
            .await
            .unwrap();

    axum::Json(reports)
}

// POST (/api/project/${project_id}/report)
pub async fn post(
    axum::extract::State(server_state): axum::extract::State<Arc<RwLock<state::ServerState>>>,
    axum::extract::Path(project_id): axum::extract::Path<String>,
    axum::extract::Json(report): axum::extract::Json<schema::Report>,
) -> impl IntoResponse {
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    let db_pool = {
        let state = server_state.read().await;
        state.db_pool.clone()
    };

    let report: schema::Report = sqlx::query_as::<_, schema::Report>("INSERT INTO report (project_id, reported_at, body, location, level, used_memory, total_memory, cpu_percent) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *")
        .bind(&uuid::Uuid::parse_str(&project_id).unwrap())
        .bind(&report.reported_at)
        .bind(&report.body)
        .bind(&report.location)
        .bind(&report.level)
        .bind(&report.used_memory)
        .bind(&report.total_memory)
        .bind(&report.cpu_percent)
        .fetch_one(&db_pool)
        .await
        .unwrap();

    axum::Json(report)
}
