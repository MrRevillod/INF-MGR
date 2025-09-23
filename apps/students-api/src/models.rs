use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Student {
    pub id: String,
    pub name: String,
    pub email: String,
    pub register: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Course {
    pub id: String,
    pub code: String,
    pub year: i32,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Enrollment {
    pub id: String,
    pub student_id: String,
    pub course_id: String,
}

impl Course {
    pub async fn get_students(
        pool: &PgPool,
        year: i32,
        code: &str,
    ) -> Result<Vec<Student>, sqlx::Error> {
        let students = sqlx::query_as::<_, Student>(
            r#"
            SELECT s.id, s.name, s.email, s.register
            FROM students s
            JOIN enrollments e ON s.id = e.student_id
            JOIN courses c ON e.course_id = c.id
            WHERE c.year = $1 AND c.code = $2
            "#,
        )
        .bind(year)
        .bind(code)
        .fetch_all(pool)
        .await?;

        Ok(students)
    }
}
