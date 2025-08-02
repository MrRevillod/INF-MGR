use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use validator::Validate;

use crate::{
    courses::Course,
    inscriptions::{Inscription, InscriptionFilter, StudentScore},
    shared::validators::validate_uuid,
};

// ============================================================================
// >>>>>>>>>>>>>>>>>>>>>>>>>> GET INSCRIPTIONS DTO <<<<<<<<<<<<<<<<<<<<<<<<<<<<
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
pub struct GetInscriptionsDto {
    #[validate(
        custom(function = validate_uuid, message = "Identificador de estudiante inválido")
    )]
    pub student_id: Option<String>,

    #[validate(
        custom(function = validate_uuid, message = "Identificador de curso inválido")
    )]
    pub course_id: Option<String>,
}

impl From<GetInscriptionsDto> for InscriptionFilter {
    fn from(dto: GetInscriptionsDto) -> Self {
        InscriptionFilter {
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
pub struct CreateInscriptionDto {
    #[validate(
        custom(function = validate_uuid, message = "Identificador de inscripción inválido")
    )]
    pub student_id: String,

    #[validate(
        custom(function = validate_uuid, message = "Identificador de curso inválido")
    )]
    pub course_id: String,
}

impl From<CreateInscriptionDto> for Inscription {
    fn from(dto: CreateInscriptionDto) -> Self {
        Inscription {
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
pub struct UpdateInscriptionDto {
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
// >>>>>>>>>>>>>>>>>>>>>>>>>> INSCRIPTION RESPONSE DTO <<<<<<<<<<<<<<<<<<<<<<<<
// ============================================================================

pub type InscriptionWithCourse = (Inscription, Course);

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct InscriptionResponse {
    pub id: Uuid,
    pub student_id: Uuid,
    pub course_id: Uuid,
    pub practice_id: Option<Uuid>,
    pub student_scores: Vec<StudentScore>,

    pub asignature: Course,
}

impl From<InscriptionWithCourse> for InscriptionResponse {
    fn from((inscription, course): InscriptionWithCourse) -> Self {
        InscriptionResponse {
            id: inscription.id,
            student_id: inscription.student_id,
            course_id: inscription.course_id,
            practice_id: inscription.practice_id,
            student_scores: inscription.student_scores,
            asignature: course,
        }
    }
}
