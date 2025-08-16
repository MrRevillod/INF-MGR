use async_trait::async_trait;
use sea_query::{Expr, ExprTrait, Order, PostgresQueryBuilder, Query};
use sea_query_sqlx::SqlxBinder;
use shaku::{Component, Interface};
use sqlx::{query_as_with as sqlx_query, Postgres};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    courses::entity::{Course, Courses},
    shared::{database::DatabaseConnection, errors::AppError},
};

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
    async fn find_many(&self, filter: CourseFilter) -> Result<Vec<Course>, AppError>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Course>, AppError>;
    async fn save(&self, course: Course) -> Result<Course, AppError>;
    async fn delete(&self, id: &Uuid) -> Result<(), AppError>;
}

#[async_trait]
impl CourseRepository for PostgresCourseRepository {
    async fn find_many(&self, filter: CourseFilter) -> Result<Vec<Course>, AppError> {
        let mut query = Query::select().expr(Expr::cust("*")).from(Courses::Table).to_owned();

        if let Some(code) = filter.code {
            query.and_where(Expr::col(Courses::Code).eq(code));
        }

        if let Some(ref name) = filter.name {
            query.and_where(Expr::col(Courses::Name).like(name));
        }

        if let Some(teacher_id) = filter.teacher_id {
            query.and_where(Expr::col(Courses::TeacherId).eq(teacher_id));
        }

        query.order_by(Courses::Year, Order::Desc);

        let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

        let result = sqlx::query_as_with::<Postgres, Course, _>(&sql, values)
            .fetch_all(self.db_connection.get_pool())
            .await?;

        Ok(result)
    }

    async fn find_by_id(&self, id: &Uuid) -> Result<Option<Course>, AppError> {
        let (sql, values) = Query::select()
            .expr(Expr::cust("*"))
            .from(Courses::Table)
            .and_where(Expr::col(Courses::Id).eq(*id))
            .build_sqlx(PostgresQueryBuilder);

        let model = sqlx_query::<Postgres, Course, _>(&sql, values)
            .fetch_optional(self.db_connection.get_pool())
            .await?;

        Ok(model)
    }

    async fn save(&self, course: Course) -> Result<Course, AppError> {
        let query = r#"
            INSERT INTO courses (id, year, code, name, course_status, teacher_id, evaluations)
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
        let (sql, values) = Query::delete()
            .from_table(Courses::Table)
            .and_where(Expr::col(Courses::Id).eq(*id))
            .build_sqlx(PostgresQueryBuilder);

        sqlx::query_with(&sql, values).execute(self.db_connection.get_pool()).await?;

        Ok(())
    }
}
