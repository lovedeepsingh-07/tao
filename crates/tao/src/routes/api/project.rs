use crate::{schema, state};
use axum::response::IntoResponse;
use std::sync::Arc;
use tokio::sync::RwLock;

// GET (/api/project/{project_id})
pub async fn get_one(
    axum::extract::State(server_state): axum::extract::State<Arc<RwLock<state::ServerState>>>,
    axum::extract::Path(project_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let db_pool = {
        let state = server_state.read().await;
        state.db_pool.clone()
    };
    let project: schema::Project =
        sqlx::query_as::<_, schema::Project>("SELECT * FROM project WHERE ID = $1")
            .bind(&project_id)
            .fetch_one(&db_pool)
            .await
            .unwrap();

    axum::Json(project)
}

// GET (/api/project)
pub async fn get_all(
    axum::extract::State(server_state): axum::extract::State<Arc<RwLock<state::ServerState>>>,
) -> impl IntoResponse {
    let db_pool = {
        let state = server_state.read().await;
        state.db_pool.clone()
    };
    let projects: Vec<schema::Project> =
        sqlx::query_as::<_, schema::Project>("SELECT * FROM project")
            .fetch_all(&db_pool)
            .await
            .unwrap();

    axum::Json(projects)
}

// POST (/api/project)
pub async fn post(
    axum::extract::State(server_state): axum::extract::State<Arc<RwLock<state::ServerState>>>,
    axum::extract::Json(project): axum::extract::Json<schema::Project>,
) -> impl IntoResponse {
    let db_pool = {
        let state = server_state.read().await;
        state.db_pool.clone()
    };

    let project: schema::Project = sqlx::query_as::<_, schema::Project>(
        "INSERT INTO project (slug, name) VALUES ($1, $2) RETURNING *",
    )
    .bind(&project.slug)
    .bind(&project.name)
    .fetch_one(&db_pool)
    .await
    .unwrap();

    axum::Json(project)
}

// DELETE (/api/project/{project_id})
pub async fn delete(
    axum::extract::State(server_state): axum::extract::State<Arc<RwLock<state::ServerState>>>,
    axum::extract::Path(project_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    let db_pool = {
        let state = server_state.read().await;
        state.db_pool.clone()
    };

    let deleted_project: schema::Project =
        sqlx::query_as::<_, schema::Project>("DELETE FROM project WHERE id = $1 RETURNING *")
            .bind(&uuid::Uuid::parse_str(&project_id).unwrap())
            .fetch_one(&db_pool)
            .await
            .unwrap();

    axum::Json(deleted_project)
}
