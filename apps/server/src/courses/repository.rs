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
    pub ids: Option<Vec<Uuid>>,
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

        if let Some(year) = filter.year {
            query.push(" AND year = ");
            query.push_bind(year);
        }

        if let Some(ids) = filter.ids {
            query.push(" AND id = ANY(");
            query.push_bind(ids);
            query.push(")");
        }

        query.push(" ORDER BY year DESC");

        let result = query
            .build_query_as::<Course>()
            .fetch_all(self.db_connection.get_pool())
            .await?;

        Ok(result)
    }

    pub async fn find_by_id(&self, id: &Uuid) -> AppResult<Option<Course>> {
        let model = sqlx::query_as::<_, Course>("SELECT * FROM courses WHERE id = $1")
            .bind(id)
            .fetch_optional(self.db_connection.get_pool())
            .await?;

        Ok(model)
    }

    pub async fn save(&self, course: Course) -> AppResult<Course> {
        let result = sqlx::query_as::<_, Course>("SELECT * FROM save_course($1)")
            .bind(course.to_json()?)
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
