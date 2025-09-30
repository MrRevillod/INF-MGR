use async_trait::async_trait;
use sea_query::{Expr, ExprTrait, PostgresQueryBuilder, Query};
use sea_query_sqlx::SqlxBinder;
use shaku::{Component, Interface};
use sqlx::{Postgres, query_as_with as sqlx_query};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    enrollments::{Enrollment, Enrollments},
    shared::{AppResult, DatabaseConnection},
};

#[derive(Component)]
#[shaku(interface = EnrollmentRepository)]
pub struct PostgresEnrollmentRepository {
    #[shaku(inject)]
    db_connection: Arc<dyn DatabaseConnection>,
}

#[derive(Default)]
pub struct EnrollmentFilter {
    pub student_id: Option<Uuid>,
    pub course_id: Option<Uuid>,
}

#[async_trait]
pub trait EnrollmentRepository: Interface {
    async fn find_many(
        &self,
        filter: EnrollmentFilter,
    ) -> AppResult<Vec<Enrollment>>;

    async fn find_by_id(&self, id: &Uuid) -> AppResult<Option<Enrollment>>;
    async fn save(&self, enrollment: Enrollment) -> AppResult<Enrollment>;
    async fn create_many(
        &self,
        enrollments: Vec<Enrollment>,
    ) -> AppResult<Vec<Enrollment>>;
    async fn delete(&self, id: &Uuid) -> AppResult<()>;
}

#[async_trait]
impl EnrollmentRepository for PostgresEnrollmentRepository {
    async fn find_many(
        &self,
        filter: EnrollmentFilter,
    ) -> AppResult<Vec<Enrollment>> {
        let mut query = Query::select()
            .expr(Expr::cust("*"))
            .from(Enrollments::Table)
            .to_owned();

        if let Some(user_id) = filter.student_id {
            query.and_where(Expr::col(Enrollments::StudentId).eq(user_id));
        }

        if let Some(course_id) = filter.course_id {
            query.and_where(Expr::col(Enrollments::CourseId).eq(course_id));
        }

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let result = sqlx_query::<Postgres, Enrollment, _>(&sql, values)
            .fetch_all(self.db_connection.get_pool())
            .await?;

        Ok(result)
    }

    async fn find_by_id(&self, id: &Uuid) -> AppResult<Option<Enrollment>> {
        let (sql, values) = Query::select()
            .expr(Expr::cust("*"))
            .from(Enrollments::Table)
            .and_where(Expr::col(Enrollments::Id).eq(*id))
            .build_sqlx(PostgresQueryBuilder);

        let model = sqlx_query::<Postgres, Enrollment, _>(&sql, values)
            .fetch_optional(self.db_connection.get_pool())
            .await?;

        Ok(model)
    }

    async fn save(&self, enrollment: Enrollment) -> AppResult<Enrollment> {
        let query = r#"
            INSERT INTO enrollments (id, student_id, course_id, practice_id, student_scores)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                practice_id = EXCLUDED.practice_id,
                student_scores = EXCLUDED.student_scores
            RETURNING *
        "#;

        let result = sqlx::query_as::<_, Enrollment>(query)
            .bind(enrollment.id)
            .bind(enrollment.student_id)
            .bind(enrollment.course_id)
            .bind(enrollment.practice_id)
            .bind(enrollment.student_scores)
            .fetch_one(self.db_connection.get_pool())
            .await?;

        Ok(result)
    }

    async fn create_many(
        &self,
        enrollments: Vec<Enrollment>,
    ) -> AppResult<Vec<Enrollment>> {
        if enrollments.is_empty() {
            return Ok(vec![]);
        }

        let mut tx = self.db_connection.get_pool().begin().await?;
        let mut created_enrollments = Vec::with_capacity(enrollments.len());

        let query = r#"
            INSERT INTO enrollments (id, student_id, course_id, practice_id, student_scores)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (student_id, course_id) DO NOTHING
            RETURNING *
        "#;

        for enrollment in enrollments {
            if let Some(created) = sqlx::query_as::<_, Enrollment>(query)
                .bind(enrollment.id)
                .bind(enrollment.student_id)
                .bind(enrollment.course_id)
                .bind(enrollment.practice_id)
                .bind(&enrollment.student_scores)
                .fetch_optional(&mut *tx)
                .await?
            {
                created_enrollments.push(created);
            }
        }

        tx.commit().await?;
        Ok(created_enrollments)
    }

    async fn delete(&self, id: &Uuid) -> AppResult<()> {
        let (sql, values) = Query::delete()
            .from_table(Enrollments::Table)
            .and_where(Expr::col(Enrollments::Id).eq(*id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values)
            .execute(self.db_connection.get_pool())
            .await?;

        Ok(())
    }
}
