use async_trait::async_trait;
use shaku::{Component, Interface};
use sqlx::{Postgres, QueryBuilder};
use std::sync::Arc;
use uuid::Uuid;

use crate::courses::Course;
use crate::shared::database::DatabaseConnection;
use crate::shared::errors::AppError;

#[derive(Component)]
#[shaku(interface = CourseRepository)]
pub struct PostgresCourseRepository {
    #[shaku(inject)]
    db_connection: Arc<dyn DatabaseConnection>,
}

#[derive(Debug, Clone, Default)]
pub struct CourseFilter {
    pub code: Option<String>,
    pub name: Option<String>,
    pub teacher_id: Option<Uuid>,
    pub year: Option<i32>,
}

#[async_trait]
pub trait CourseRepository: Interface {
    async fn find(&self, filter: CourseFilter) -> Result<Vec<Course>, AppError>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Course>, AppError>;
    async fn save(&self, course: Course) -> Result<Course, AppError>;
    async fn delete(&self, id: &Uuid) -> Result<(), AppError>;
}

#[async_trait]
impl CourseRepository for PostgresCourseRepository {
    async fn find(&self, filter: CourseFilter) -> Result<Vec<Course>, AppError> {
        let mut builder =
            QueryBuilder::<Postgres>::new("SELECT * FROM Courses WHERE 1=1");

        if let Some(ref code) = filter.code {
            builder.push(" AND code = ").push_bind(code);
        }

        if let Some(ref name) = filter.name {
            builder.push(" AND name = ").push_bind(name);
        }

        if let Some(teacher_id) = filter.teacher_id {
            builder.push(" AND teacher_id = ").push_bind(teacher_id);
        }

        let query = builder.build_query_as::<Course>();

        Ok(query.fetch_all(self.db_connection.get_pool()).await?)
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Course>, AppError> {
        let query = r#"SELECT * FROM Courses WHERE id = $1"#;

        let model = sqlx::query_as::<_, Course>(query)
            .bind(id)
            .fetch_optional(self.db_connection.get_pool())
            .await?;

        Ok(model)
    }

    async fn save(&self, course: Course) -> Result<Course, AppError> {
        let query = r#"
            INSERT INTO Courses (id, year, code, name, course_status, teacher_id, evaluations)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                teacher_id = EXCLUDED.teacher_id,
                course_status = EXCLUDED.course_status,
                evaluations = EXCLUDED.evaluations
            RETURNING *
        "#;

        let result = sqlx::query_as::<_, Course>(query)
            .bind(course.id)
            .bind(course.year)
            .bind(&course.code)
            .bind(&course.name)
            .bind(course.course_status)
            .bind(course.teacher_id)
            .bind(&course.evaluations)
            .fetch_one(self.db_connection.get_pool())
            .await?;

        Ok(result)
    }

    async fn delete(&self, id: &Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM Courses WHERE id = $1")
            .bind(id)
            .execute(self.db_connection.get_pool())
            .await?;

        Ok(())
    }
}
