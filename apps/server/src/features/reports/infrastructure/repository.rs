use sqlx::{Postgres, QueryBuilder};
use std::sync::Arc;
use uuid::Uuid;

use async_trait::async_trait;
use shaku::Component;

use crate::{
    reports::{
        domain::{Report, ReportError, ReportFilter, ReportRepository},
        infrastructure::models::ReportModel,
    },
    shared::database::DatabaseConnection,
};

#[derive(Component)]
#[shaku(interface = ReportRepository)]
pub struct PostgresReportRepository {
    #[shaku(inject)]
    db_connection: Arc<dyn DatabaseConnection>,
}

#[async_trait]
impl ReportRepository for PostgresReportRepository {
    async fn find_all(
        &self,
        filter: ReportFilter,
    ) -> Result<Vec<Report>, ReportError> {
        let mut query =
            QueryBuilder::<Postgres>::new("SELECT * FROM reports WHERE 1=1");

        if let Some(inscription_id) = filter.inscription_id {
            query.push(" AND student_id = ");
            query.push_bind(inscription_id);
        }

        query.push(" ORDER BY created_at DESC");

        let reports = query
            .build_query_as::<ReportModel>()
            .fetch_all(self.db_connection.get_pool())
            .await?;

        Ok(reports.into_iter().map(Report::from).collect())
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Report>, ReportError> {
        let report = sqlx::query_as::<Postgres, ReportModel>(
            "SELECT * FROM reports WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(self.db_connection.get_pool())
        .await?;

        Ok(report.map(Report::from))
    }

    async fn create_many(&self, reports: Vec<Report>) -> Result<(), ReportError> {
        let mut query = QueryBuilder::<Postgres>::new(
            r#"INSERT INTO reports (id, student_id, title, content, created_at, updated_at) 
            VALUES"#,
        );

        query.push_values(reports, |mut b, report| {
            b.push_bind(report.id)
                .push_bind(report.student_id)
                .push_bind(report.title)
                .push_bind(report.content)
                .push_bind(report.created_at)
                .push_bind(report.updated_at);
        });

        query.build().execute(self.db_connection.get_pool()).await?;

        Ok(())
    }

    async fn update(&self, report: Report) -> Result<Report, ReportError> {
        let query = r#"
            UPDATE reports 
            SET title = $1, content = $2, updated_at = $3 
            WHERE id = $4
            RETURNING *
        "#;

        let updated_report = sqlx::query_as::<Postgres, ReportModel>(query)
            .bind(report.title)
            .bind(report.content)
            .bind(report.updated_at)
            .bind(report.id)
            .fetch_one(self.db_connection.get_pool())
            .await?;

        Ok(Report::from(updated_report))
    }
}
