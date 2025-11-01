use sword::core::injectable;

use crate::{
    courses::Course,
    shared::{AppResult, PostgresDatabase},
    types::*,
};

#[injectable]
pub struct CourseRepository {
    db_connection: Arc<PostgresDatabase>,
}

#[derive(Debug, Clone, Default)]
pub struct CourseFilter {
    pub code: Option<String>,
    pub name: Option<String>,
    pub teacher_id: Option<Uuid>,
    pub year: Option<i32>,
}

impl CourseRepository {
    pub async fn find_many(&self, filter: CourseFilter) -> AppResult<Vec<Course>> {
        let mut query = QueryBuilder::new("SELECT * FROM courses WHERE 1=1");

        if let Some(code) = filter.code {
            query.push(" AND code = ");
            query.push_bind(code);
        }

        if let Some(ref name) = filter.name {
            query.push(" AND name ILIKE ");
            query.push_bind(format!("%{name}%"));
        }

        if let Some(teacher_id) = filter.teacher_id {
            query.push(" AND teacher_id = ");
            query.push_bind(teacher_id);
        }

        query.push(" ORDER BY year DESC");

        let result = query
            .build_query_as::<Course>()
            .fetch_all(self.db_connection.get_pool())
            .await?;

        Ok(result)
    }

    pub async fn find_by_id(&self, id: &Uuid) -> AppResult<Option<Course>> {
        let model =
            sqlx::query_as::<_, Course>("SELECT * FROM courses WHERE id = $1")
                .bind(id)
                .fetch_optional(self.db_connection.get_pool())
                .await?;

        Ok(model)
    }

    pub async fn save(&self, course: Course) -> AppResult<Course> {
        let query = r"
            INSERT INTO courses (id, year, code, name, course_status, teacher_id, evaluations)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                teacher_id = EXCLUDED.teacher_id,
                course_status = EXCLUDED.course_status,
                evaluations = EXCLUDED.evaluations
            RETURNING *
        ";

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

    pub async fn delete(&self, id: &Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM courses WHERE id = $1")
            .bind(id)
            .execute(self.db_connection.get_pool())
            .await?;

        Ok(())
    }
}
