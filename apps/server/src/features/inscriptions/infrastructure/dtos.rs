use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::shared::validators::validate_uuid;
use validators::validate_student_state;

use crate::inscriptions::{
    application::UpdateInscriptionInput,
    domain::{Inscription, InscriptionFilter, StudentEvaluation},
};

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateInscriptionDto {
    #[validate(custom(function = validate_uuid, message = "Identificador de inscripción inválido"))]
    pub user_id: String,

    #[validate(custom(function = validate_uuid, message = "Identificador de asignatura inválido"))]
    pub asignature_id: String,
}

impl From<CreateInscriptionDto> for Inscription {
    fn from(value: CreateInscriptionDto) -> Self {
        Inscription {
            id: Uuid::new_v4(),
            user_id: Uuid::parse_str(&value.user_id).unwrap(),
            asignature_id: Uuid::parse_str(&value.asignature_id).unwrap(),
            practice_id: None,
            evaluations_scores: vec![],
            status: "active".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInscriptionDto {
    #[validate(nested)]
    pub evaluations_scores: Option<Vec<StudentEvaluationDto>>,

    #[validate(length(min = 1, message = "El estado no puede estar vacío"))]
    pub status: Option<String>,

    #[validate(custom(function = validate_uuid, message = "Identificador de práctica inválido"))]
    pub practice_id: Option<String>,
}

impl From<UpdateInscriptionDto> for UpdateInscriptionInput {
    fn from(value: UpdateInscriptionDto) -> Self {
        UpdateInscriptionInput {
            practice_id: value.practice_id.map(|id| Uuid::parse_str(&id).unwrap()),
            evaluations_scores: value
                .evaluations_scores
                .map(|scores| scores.into_iter().map(Into::into).collect()),

            status: value.status,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct StudentEvaluationDto {
    #[validate(custom(function = validate_uuid, message = "Identificador de evaluación inválido"))]
    pub id: String,

    #[validate(range(
        min = 1.0,
        max = 7.0,
        message = "La puntuación debe estar entre 0 y 100"
    ))]
    pub score: f64,
}

impl From<StudentEvaluationDto> for StudentEvaluation {
    fn from(dto: StudentEvaluationDto) -> Self {
        StudentEvaluation {
            id: Uuid::parse_str(&dto.id).unwrap(),
            score: dto.score,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, Default)]
pub struct InscriptionQueryDto {
    #[validate(custom(function = validate_uuid, message = "Identificador de usuario inválido"))]
    pub user_id: Option<String>,

    #[validate(custom(function = validate_uuid, message = "Identificador de asignatura inválido"))]
    pub asignature_id: Option<String>,

    #[validate(
        custom(function = validate_student_state),
        length(min = 1, message = "El estado no puede estar vacío")
    )]
    pub status: Option<String>,
}

impl From<InscriptionQueryDto> for InscriptionFilter {
    fn from(dto: InscriptionQueryDto) -> Self {
        InscriptionFilter {
            user_id: dto.user_id.map(|id| Uuid::parse_str(&id).unwrap()),
            asignature_id: dto.asignature_id.map(|id| Uuid::parse_str(&id).unwrap()),
            status: dto.status,
        }
    }
}

mod validators {
    use validator::ValidationError;

    pub fn validate_student_state(state: &str) -> Result<(), ValidationError> {
        match state.to_lowercase().as_str() {
            "active" | "inactive" | "completed" | "evaluating" => Ok(()),
            _ => Err(ValidationError::new("Estado de inscripción inválido")),
        }
    }
}
