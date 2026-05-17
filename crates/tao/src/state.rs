use sqlx::{migrate, postgres};

static MIGRATOR: migrate::Migrator = sqlx::migrate!("../../migrations");

#[derive(Debug, Clone)]
pub struct ServerState {
    pub env: crate::Environment,
    pub db_pool: postgres::PgPool,
}

impl ServerState {
    pub async fn new(db_url: &str, env: crate::Environment) -> Result<Self, error::Error> {
        let db_pool = postgres::PgPool::connect(db_url).await?;
        MIGRATOR.run(&db_pool).await?;
        Ok(Self { env, db_pool })
    }
}
