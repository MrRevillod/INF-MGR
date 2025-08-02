use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Inscription {
    pub id: Uuid,

    #[sqlx(rename = "student_id")]
    pub student_id: Uuid,

    #[sqlx(rename = "course_id")]
    pub course_id: Uuid,

    #[sqlx(rename = "practice_id")]
    pub practice_id: Option<Uuid>,

    #[sqlx(rename = "student_scores")]
    pub student_scores: Vec<StudentScore>,
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "student_score")]
#[serde(rename_all = "camelCase")]
pub struct StudentScore {
    pub evaluation_id: Uuid,
    pub score: f64,
}
