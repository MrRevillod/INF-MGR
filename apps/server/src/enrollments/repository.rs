use async_trait::async_trait;
use sea_query::{Expr, ExprTrait, PostgresQueryBuilder, Query};
use sea_query_sqlx::SqlxBinder;
use shaku::{Component, Interface};
use sqlx::{query_as_with as sqlx_query, Postgres};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    enrollments::{entity::Enrollments, Enrollment},
    shared::{database::DatabaseConnection, errors::AppError},
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
    ) -> Result<Vec<Enrollment>, AppError>;

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Enrollment>, AppError>;
    async fn save(&self, enrollment: Enrollment) -> Result<Enrollment, AppError>;
    async fn create_many(
        &self,
        enrollments: Vec<Enrollment>,
    ) -> Result<Vec<Enrollment>, AppError>;
    async fn delete(&self, id: &Uuid) -> Result<(), AppError>;
}

#[async_trait]
impl EnrollmentRepository for PostgresEnrollmentRepository {
    async fn find_many(
        &self,
        filter: EnrollmentFilter,
    ) -> Result<Vec<Enrollment>, AppError> {
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

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Enrollment>, AppError> {
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

    async fn save(&self, enrollment: Enrollment) -> Result<Enrollment, AppError> {
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
    ) -> Result<Vec<Enrollment>, AppError> {
        if enrollments.is_empty() {
            return Ok(vec![]);
        }

        let mut query_values = Vec::new();
        let mut arg_index = 1;

        for _enrollment in &enrollments {
            query_values.push(format!(
                "({}, {}, {}, {}, {})",
                format!("${}", arg_index),     // id
                format!("${}", arg_index + 1), // student_id
                format!("${}", arg_index + 2), // course_id
                format!("${}", arg_index + 3), // practice_id
                format!("${}", arg_index + 4)  // student_scores
            ));
            arg_index += 5;
        }

        let query = format!(
            r#"
            INSERT INTO enrollments (id, student_id, course_id, practice_id, student_scores)
            VALUES {}
            ON CONFLICT (id) DO NOTHING
            RETURNING *
            "#,
            query_values.join(", ")
        );

        let mut sqlx_query = sqlx::query_as::<_, Enrollment>(&query);

        // Bind all parameters for all enrollments
        for enrollment in &enrollments {
            sqlx_query = sqlx_query
                .bind(&enrollment.id)
                .bind(&enrollment.student_id)
                .bind(&enrollment.course_id)
                .bind(&enrollment.practice_id)
                .bind(&enrollment.student_scores);
        }

        let results = sqlx_query.fetch_all(self.db_connection.get_pool()).await?;

        Ok(results)
    }

    async fn delete(&self, id: &Uuid) -> Result<(), AppError> {
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
