mod project;
mod report;

pub use project::delete as project_delete;
pub use project::get_all as project_get_all;
pub use project::get_one as project_get_one;
pub use project::post as project_post;
pub use report::get as report_get;
pub use report::post as report_post;

use crate::state;
use axum::response::IntoResponse;
use std::sync::Arc;
use tokio::sync::RwLock;

// GET (/api/health)
pub async fn health(
    axum::extract::State(server_state): axum::extract::State<Arc<RwLock<state::ServerState>>>,
) -> impl IntoResponse {
    let db_pool = {
        let state = server_state.read().await;
        state.db_pool.clone()
    };
    match sqlx::query_as::<_, (i32,)>("SELECT 1")
        .fetch_one(&db_pool)
        .await
    {
		// TODO: remove this 'type shi'
        Ok(_) => {
            // TODO: simulating latency, only for development
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            "type shi".to_string()
        }
        Err(e) => {
            log::error!("Health check failed: {}", e);
            "DB ERROR".to_string()
        }
    }
}
