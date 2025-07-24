use async_trait::async_trait;
use shaku::Component;
use std::sync::Arc;

use crate::reports::{
    application::GetReportsCase,
    domain::{Report, ReportError, ReportFilter, ReportRepository},
};

#[derive(Component)]
#[shaku(interface = GetReportsCase)]
pub struct GetReportsCaseImpl {
    #[shaku(inject)]
    repository: Arc<dyn ReportRepository>,
}

#[async_trait]
impl GetReportsCase for GetReportsCaseImpl {
    async fn execute(
        &self,
        filter: ReportFilter,
    ) -> Result<Vec<Report>, ReportError> {
        self.repository.find_all(filter).await
    }
}
