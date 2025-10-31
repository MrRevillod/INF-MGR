use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Enrollment {
    pub id: Uuid,
    pub student_id: Uuid,
    pub course_id: Uuid,
    pub practice_id: Option<Uuid>,
    pub student_scores: Vec<StudentScore>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "student_score")]
#[serde(rename_all = "camelCase")]
pub struct StudentScore {
    pub evaluation_id: Uuid,
    pub score: f64,
}
