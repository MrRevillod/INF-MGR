use sword::prelude::*;
use uuid::Uuid;

use crate::{
    reports::{
        application::{GetReportsCase, UpdateReportCase},
        infrastructure::{dtos::GetReportsQuery, ReportModel, UpdateReportDto},
    },
    shared::di::AppModule,
};

#[controller("/reports")]
pub struct ReportController {}

#[routes]
impl ReportController {
    #[get("/")]
    async fn get_reports(ctx: Context) -> HttpResult<HttpResponse> {
        let filter = ctx.validated_query_optional::<GetReportsQuery>()?;
        let use_case = ctx.get_dependency::<AppModule, dyn GetReportsCase>()?;

        let reports = use_case.execute(filter.unwrap_or_default().into()).await?;

        let reports = reports
            .into_iter()
            .map(ReportModel::from)
            .collect::<Vec<_>>();

        Ok(HttpResponse::Ok().data(reports))
    }

    #[patch("/{id}")]
    async fn update_report(ctx: Context) -> HttpResult<HttpResponse> {
        let id = ctx.param::<Uuid>("id")?;
        let body = ctx.validated_body::<UpdateReportDto>()?;

        let use_case = ctx.get_dependency::<AppModule, dyn UpdateReportCase>()?;
        let report = use_case.execute(&id, body.into()).await?;

        Ok(HttpResponse::Ok().data(ReportModel::from(report)))
    }
}
