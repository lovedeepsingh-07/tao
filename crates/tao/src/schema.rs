#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
pub struct Project {
    #[serde(default)]
    pub id: uuid::Uuid,
    pub slug: String,
    pub name: String,
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReportKind {
    #[default]
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

    pub release: String,

    pub body: String,
    pub stack_trace: Option<String>,
    pub kind: ReportKind,

    pub memory_usage: i32,
    pub cpu_percent: f64,
    pub disk_usage_percent: f64,
}
