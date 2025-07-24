use async_trait::async_trait;
use chrono::{DateTime, Utc};
use shaku::Interface;
use thiserror::Error;
use uuid::Uuid;

pub struct Report {
    pub id: Uuid,
    pub student_id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct ReportFilter {
    pub inscription_id: Option<Uuid>,
}

#[async_trait]
pub trait ReportRepository: Interface {
    async fn find_all(
        &self,
        filter: ReportFilter,
    ) -> Result<Vec<Report>, ReportError>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Report>, ReportError>;
    async fn create_many(&self, reports: Vec<Report>) -> Result<(), ReportError>;
    async fn update(&self, report: Report) -> Result<Report, ReportError>;
}

#[derive(Debug, Error)]
pub enum ReportError {
    #[error("Report not found")]
    NotFound,

    #[error("Reports database error: {source}")]
    Database {
        #[from]
        source: sqlx::Error,
    },
}
