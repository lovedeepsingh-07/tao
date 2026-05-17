#[derive(Debug, Default, serde::Serialize, serde::Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
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
    #[serde(default)]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub body: String,
    pub kind: ReportKind,
}
