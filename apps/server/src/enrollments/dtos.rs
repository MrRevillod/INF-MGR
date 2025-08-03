use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    enrollments::{Enrollment, EnrollmentFilter, StudentScore},
    shared::validators::validate_uuid,
    users::User,
};

// ============================================================================
// >>>>>>>>>>>>>>>>>>>>>>>>>> GET INSCRIPTIONS DTO <<<<<<<<<<<<<<<<<<<<<<<<<<<<
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
pub struct GetEnrollmentsDto {
    #[validate(
        custom(function = validate_uuid, message = "Identificador de estudiante inválido")
    )]
    pub student_id: Option<String>,

    #[validate(
        custom(function = validate_uuid, message = "Identificador de curso inválido")
    )]
    pub course_id: Option<String>,
}

impl From<GetEnrollmentsDto> for EnrollmentFilter {
    fn from(dto: GetEnrollmentsDto) -> Self {
        EnrollmentFilter {
            student_id: dto.student_id.map(|id| Uuid::parse_str(&id).unwrap()),
            course_id: dto.course_id.map(|id| Uuid::parse_str(&id).unwrap()),
        }
    }
}

// ============================================================================
// >>>>>>>>>>>>>>>>>>>>>>>>>> CREATE INSCRIPTION DTO <<<<<<<<<<<<<<<<<<<<<<<<<<
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateEnrollmentDto {
    #[validate(
        custom(function = validate_uuid, message = "Identificador de inscripción inválido")
    )]
    pub student_id: String,

    #[validate(
        custom(function = validate_uuid, message = "Identificador de curso inválido")
    )]
    pub course_id: String,
}

impl From<CreateEnrollmentDto> for Enrollment {
    fn from(dto: CreateEnrollmentDto) -> Self {
        Enrollment {
            id: Uuid::new_v4(),
            student_id: Uuid::parse_str(&dto.student_id).unwrap(),
            course_id: Uuid::parse_str(&dto.course_id).unwrap(),
            student_scores: vec![],
            practice_id: None,
        }
    }
}

// ============================================================================
// >>>>>>>>>>>>>>>>>>>>>>>>>> UPDATE INSCRIPTION DTO <<<<<<<<<<<<<<<<<<<<<<<<<<
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEnrollmentDto {
    #[validate(nested)]
    pub student_scores: Option<Vec<StudentScoreDto>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct StudentScoreDto {
    #[validate(
        custom(function = validate_uuid, message = "Identificador de evaluación inválido")
    )]
    pub evaluation_id: String,

    #[validate(range(
        min = 1.0,
        max = 7.0,
        message = "La puntuación debe estar entre 0 y 100"
    ))]
    pub score: f64,
}

impl From<StudentScoreDto> for StudentScore {
    fn from(dto: StudentScoreDto) -> Self {
        StudentScore {
            evaluation_id: Uuid::parse_str(&dto.evaluation_id).unwrap(),
            score: dto.score,
        }
    }
}

// ============================================================================
// >>>>>>>>>>>>>>>>>>>>>>>>>> Enrollment Response DTO <<<<<<<<<<<<<<<<<<<<<<<<<
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EnrollmentResponse {
    pub id: String,
    pub student_id: String,
    pub course_id: String,
    pub student_scores: Vec<StudentScore>,
    pub practice_id: Option<String>,

    pub student: User,
}

pub type EnrollmentWithStudent = (Enrollment, User);

impl From<EnrollmentWithStudent> for EnrollmentResponse {
    fn from((enrollment, student): EnrollmentWithStudent) -> Self {
        EnrollmentResponse {
            id: enrollment.id.to_string(),
            student_id: enrollment.student_id.to_string(),
            course_id: enrollment.course_id.to_string(),
            student_scores: enrollment.student_scores,
            practice_id: enrollment.practice_id.map(|id| id.to_string()),
            student,
        }
    }
}
