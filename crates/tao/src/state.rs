use sqlx::postgres;

#[derive(Debug, Clone)]
pub struct ServerState {
    pub env: crate::Environment,
    pub db_pool: postgres::PgPool,
}

impl ServerState {
    pub async fn new(db_url: &str, env: crate::Environment) -> Result<Self, error::Error> {
        let db_pool = postgres::PgPool::connect(db_url).await?;
        Ok(Self { env, db_pool })
    }
}
