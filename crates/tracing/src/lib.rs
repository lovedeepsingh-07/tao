pub use tao_schema as schema;

use reqwest as rq;
use tokio::sync::mpsc;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Default)]
struct EventVisitor {
    message: Option<String>,
}

impl tracing::field::Visit for EventVisitor {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.message = Some(value.to_string());
        }
    }
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        if field.name() == "message" {
            self.message = Some(format!("{value:?}"));
        }
    }
}

pub struct TaoTracingLayer {
    root_crate: String,
    project_id: uuid::Uuid,
    tx: mpsc::Sender<schema::Report>,
}
impl TaoTracingLayer {
    pub fn new(
        root_crate: String,
        project_id: uuid::Uuid,
        tx: mpsc::Sender<schema::Report>,
    ) -> Self {
        Self {
            root_crate,
            project_id,
            tx,
        }
    }
}

impl<S> tracing_subscriber::layer::Layer<S> for TaoTracingLayer
where
    S: tracing::Subscriber,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let event_metadata = event.metadata();
        if !event_metadata.target().starts_with(&self.root_crate) {
            println!("{}", event_metadata.target());
            return;
        }
        let report_level = match *event_metadata.level() {
            tracing::Level::TRACE => schema::ReportLevel::TRACE,
            tracing::Level::DEBUG => schema::ReportLevel::DEBUG,
            tracing::Level::INFO => schema::ReportLevel::INFO,
            tracing::Level::WARN => schema::ReportLevel::WARN,
            tracing::Level::ERROR => schema::ReportLevel::ERROR,
        };

        let mut visitor = EventVisitor::default();
        event.record(&mut visitor);
        let body = visitor.message.unwrap_or_default();

        let _ = self.tx.try_send(schema::Report {
            id: uuid::Uuid::new_v4(),
            project_id: self.project_id,
            reported_at: chrono::Utc::now(),
            body,
            location: format!(
                "{}:{}:{}",
                event_metadata.module_path().unwrap(),
                event_metadata.file().unwrap(),
                event_metadata.line().unwrap()
            ),
            level: report_level,
            ..Default::default()
        });
    }
}

pub async fn init<S>(root_crate: String, api_url: String, project_id: uuid::Uuid, subscriber: S)
where
    S: tracing::Subscriber + Send + Sync + 'static,
{
    let (tx, mut rx) = mpsc::channel::<tao_schema::Report>(1028);

    subscriber
        .with(TaoTracingLayer::new(root_crate, project_id, tx))
        .init();

    tokio::spawn(async move {
        let client = rq::Client::new();
        let mut system = sysinfo::System::new_all();

        while let Some(mut report) = rx.recv().await {
            system.refresh_memory();
            system.refresh_cpu_usage();
            report.used_memory = (system.used_memory() / 1000) as i32;
            report.total_memory = (system.total_memory() / 1000) as i32;
            report.cpu_percent = system.global_cpu_usage() as f64;
            client
                .post(&format!("{}/api/project/{}/report", &api_url, project_id))
                .json(&report)
                .send()
                .await
                .unwrap();
        }
    });
}
