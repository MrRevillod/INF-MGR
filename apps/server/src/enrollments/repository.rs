use sword::core::injectable;

use crate::{
    enrollments::Enrollment,
    shared::{AppResult, PostgresDatabase},
    types::*,
};

#[injectable]
pub struct EnrollmentRepository {
    db: Arc<PostgresDatabase>,
}

#[derive(Default)]
pub struct EnrollmentFilter {
    pub student_id: Option<Uuid>,
    pub course_id: Option<Uuid>,
}

impl EnrollmentRepository {
    pub async fn find_many(
        &self,
        filter: EnrollmentFilter,
    ) -> AppResult<Vec<Enrollment>> {
        let mut query = QueryBuilder::new("SELECT * FROM enrollments WHERE 1=1");

        if let Some(user_id) = filter.student_id {
            query.push(" AND student_id = ");
            query.push_bind(user_id);
        }

        if let Some(course_id) = filter.course_id {
            query.push(" AND course_id = ");
            query.push_bind(course_id);
        }

        let result = query
            .build_query_as::<Enrollment>()
            .fetch_all(self.db.get_pool())
            .await?;

        Ok(result)
    }

    pub async fn find_by_id(&self, id: &Uuid) -> AppResult<Option<Enrollment>> {
        let model =
            sqlx::query_as::<_, Enrollment>("SELECT * FROM enrollments WHERE id = $1")
                .bind(id)
                .fetch_optional(self.db.get_pool())
                .await?;

        Ok(model)
    }

    pub async fn save(&self, enrollment: Enrollment) -> AppResult<Enrollment> {
        let query = r"
            INSERT INTO enrollments (id, student_id, course_id, practice_id, student_scores)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (id) DO UPDATE SET
                practice_id = EXCLUDED.practice_id,
                student_scores = EXCLUDED.student_scores
            RETURNING *
        ";

        let result = sqlx::query_as::<_, Enrollment>(query)
            .bind(enrollment.id)
            .bind(enrollment.student_id)
            .bind(enrollment.course_id)
            .bind(enrollment.practice_id)
            .bind(enrollment.student_scores)
            .fetch_one(self.db.get_pool())
            .await?;

        Ok(result)
    }

    pub async fn create_many(
        &self,
        enrollments: Vec<Enrollment>,
    ) -> AppResult<Vec<Enrollment>> {
        if enrollments.is_empty() {
            return Ok(vec![]);
        }

        let mut tx = self.db.tx_begin().await?;
        let mut created_enrollments = Vec::with_capacity(enrollments.len());

        let query = r"
            INSERT INTO enrollments (id, student_id, course_id, practice_id, student_scores)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (student_id, course_id) DO NOTHING
            RETURNING *
        ";

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

    pub async fn delete(&self, id: &Uuid) -> AppResult<()> {
        sqlx::query("DELETE FROM enrollments WHERE id = $1")
            .bind(id)
            .execute(self.db.get_pool())
            .await?;

        Ok(())
    }
}
