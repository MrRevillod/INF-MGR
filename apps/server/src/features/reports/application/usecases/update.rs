use async_trait::async_trait;
use chrono::Utc;
use shaku::Component;
use std::sync::Arc;
use uuid::Uuid;

use crate::reports::{
    application::{UpdateReportCase, UpdateReportInput},
    domain::{Report, ReportError, ReportRepository},
};

#[derive(Component)]
#[shaku(interface = UpdateReportCase)]
pub struct UpdateReportCaseImpl {
    #[shaku(inject)]
    repository: Arc<dyn ReportRepository>,
}

#[async_trait]
impl UpdateReportCase for UpdateReportCaseImpl {
    async fn execute(
        &self,
        id: &Uuid,
        input: UpdateReportInput,
    ) -> Result<Report, ReportError> {
        let Some(mut report) = self.repository.find_by_id(id).await? else {
            return Err(ReportError::NotFound);
        };

        if let Some(title) = input.title {
            report.title = title;
        }

        if let Some(content) = input.content {
            report.content = content;
        }

        report.updated_at = Utc::now();

        self.repository.update(report).await
    }
}
