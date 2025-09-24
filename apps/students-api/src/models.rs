use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Student {
    pub id: String,
    pub name: String,
    pub rut: String,
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

impl Student {
    /// Formats RUT to standard format: without dots, with dash
    /// Examples: "12.345.678-9" -> "12345678-9", "12345678-9" -> "12345678-9"
    pub fn format_rut(rut: &str) -> String {
        let cleaned: String = rut
            .chars()
            .filter(|c| c.is_ascii_digit() || *c == '-')
            .collect();

        if !cleaned.contains('-') && cleaned.len() >= 2 {
            let (numbers, check_digit) = cleaned.split_at(cleaned.len() - 1);
            format!("{}-{}", numbers, check_digit)
        } else {
            cleaned
        }
    }

    pub fn with_formatted_rut(mut self) -> Self {
        self.rut = Self::format_rut(&self.rut);
        self
    }
}

impl Course {
    pub async fn get_students(
        pool: &PgPool,
        year: i32,
        code: &str,
    ) -> Result<Vec<Student>, sqlx::Error> {
        let result = sqlx::query_as::<_, Student>(
            r#"
                SELECT s.id, s.name, s.rut, s.email, s.register
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

        let formatted_students = result
            .into_iter()
            .map(|student| student.with_formatted_rut())
            .collect();

        Ok(formatted_students)
    }
}
