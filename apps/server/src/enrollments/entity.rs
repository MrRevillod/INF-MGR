use sea_query::Iden;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, FromRow)]
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

pub enum Enrollments {
    Table,
    Id,
    StudentId,
    CourseId,
    PracticeId,
    StudentScores,
}

impl Iden for Enrollments {
    fn unquoted(&self) -> &str {
        match self {
            Enrollments::Table => "enrollments",
            Enrollments::Id => "id",
            Enrollments::StudentId => "student_id",
            Enrollments::CourseId => "course_id",
            Enrollments::PracticeId => "practice_id",
            Enrollments::StudentScores => "student_scores",
        }
    }
}

pub const ENROLLMENT_INSERT_COLUMNS: [Enrollments; 5] = [
    Enrollments::Id,
    Enrollments::StudentId,
    Enrollments::CourseId,
    Enrollments::PracticeId,
    Enrollments::StudentScores,
];

pub const ENROLLMENT_UPDATE_COLUMNS: [Enrollments; 1] = [Enrollments::StudentScores];
