use async_trait::async_trait;
use shaku::{Component, Interface};
use sqlx::Postgres;
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    enrollments::Enrollment,
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
    async fn find_all(
        &self,
        filter: EnrollmentFilter,
    ) -> Result<Vec<Enrollment>, AppError>;

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Enrollment>, AppError>;
    async fn save(&self, enrollment: Enrollment) -> Result<Enrollment, AppError>;
    async fn delete(&self, id: &Uuid) -> Result<(), AppError>;
}

#[async_trait]
impl EnrollmentRepository for PostgresEnrollmentRepository {
    async fn find_all(
        &self,
        filter: EnrollmentFilter,
    ) -> Result<Vec<Enrollment>, AppError> {
        let mut builder = sqlx::QueryBuilder::<Postgres>::new(
            "SELECT * FROM enrollments WHERE 1=1",
        );

        if let Some(user_id) = filter.student_id {
            builder.push(" AND student_id = ").push_bind(user_id);
        }

        if let Some(course_id) = filter.course_id {
            builder.push(" AND course_id = ").push_bind(course_id);
        }

        let query = builder.build_query_as::<Enrollment>();
        let result = query.fetch_all(self.db_connection.get_pool()).await?;

        Ok(result)
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Enrollment>, AppError> {
        let query = r#"SELECT * FROM enrollments WHERE id = $1"#;

        let model = sqlx::query_as::<_, Enrollment>(query)
            .bind(id)
            .fetch_optional(self.db_connection.get_pool())
            .await?;

        Ok(model)
    }

    async fn save(&self, enrollment: Enrollment) -> Result<Enrollment, AppError> {
        let query = r#"
            INSERT INTO enrollments (id, student_id, course_id, practice_id, student_scores)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                student_id = EXCLUDED.student_id,
                course_id = EXCLUDED.course_id,
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

    async fn delete(&self, id: &Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM enrollments WHERE id = $1")
            .bind(id)
            .execute(self.db_connection.get_pool())
            .await?;

        Ok(())
    }
}
