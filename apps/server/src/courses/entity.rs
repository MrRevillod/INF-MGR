use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Course {
    pub id: Uuid,
    pub year: i32,
    pub code: String,
    pub name: String,
    pub status: CourseStatus,
    pub evaluations: Vec<CourseEvaluation>,

    #[sqlx(rename = "teacher_id")]
    pub teacher_id: Uuid,

    #[sqlx(rename = "coordinator_id")]
    pub coordinator_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "course_evaluation")]
pub struct CourseEvaluation {
    pub id: Uuid,
    pub name: String,
    pub weight: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "course_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum CourseStatus {
    InProgress,
    Ended,
}
