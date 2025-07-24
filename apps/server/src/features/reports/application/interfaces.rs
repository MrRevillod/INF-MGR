use async_trait::async_trait;
use shaku::Interface;
use uuid::Uuid;

use crate::reports::domain::{Report, ReportError, ReportFilter};

#[async_trait]
pub trait GetReportsCase: Interface {
    async fn execute(
        &self,
        filter: ReportFilter,
    ) -> Result<Vec<Report>, ReportError>;
}

pub struct UpdateReportInput {
    pub title: Option<String>,
    pub content: Option<String>,
}

#[async_trait]
pub trait UpdateReportCase: Interface {
    async fn execute(
        &self,
        id: &Uuid,
        report: UpdateReportInput,
    ) -> Result<Report, ReportError>;
}
