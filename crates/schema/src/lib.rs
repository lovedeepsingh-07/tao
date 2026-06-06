#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Project {
    #[serde(default)]
    pub id: uuid::Uuid,
    pub name: String,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReportLevel {
    #[default]
    TRACE,
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Report {
    #[serde(default)]
    pub id: uuid::Uuid,
    pub project_id: uuid::Uuid,

    #[serde(default)]
    pub reported_at: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub received_at: chrono::DateTime<chrono::Utc>,

    pub body: String,
    pub location: String,
    pub level: ReportLevel,

    pub memory_usage: i32,
    pub cpu_percent: f64,
}
