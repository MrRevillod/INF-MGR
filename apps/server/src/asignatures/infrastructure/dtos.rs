use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use validators::ASIGNATURE_CODE_REGEX;

use crate::asignatures::{
    application::{CreateAsignatureInput, UpdateAsignatureInput},
    domain::{AsignatureError, Evaluation},
    infrastructure::dtos::validators::validate_evaluation_weights,
};

use crate::shared::validators::validate_uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CreateAsignatureDto {
    #[validate(range(
        min = 2000,
        max = 2100,
        message = "El año debe tener 4 dígitos."
    ))]
    pub year: i16,

    #[validate(
        length(equal = 7, message = "El código debe tener 7 caracteres."),
        regex(
            path = *ASIGNATURE_CODE_REGEX,
            message = "El código debe seguir el formato INFO{NNNN}."
        )
    )]
    pub code: String,

    #[validate(length(
        min = 1,
        max = 100,
        message = "El nombre debe tener entre 1 y 100 caracteres."
    ))]
    pub name: String,

    #[validate(
        nested,
        length(min = 1, message = "Debe haber al menos una evaluación."),
        custom(function = validate_evaluation_weights)
    )]
    pub evaluations: Vec<EvaluationDto>,

    #[validate(custom(function = validate_uuid))]
    pub teacher_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct EvaluationDto {
    #[validate(length(
        min = 1,
        max = 100,
        message = "El nombre de la evaluación debe tener entre 1 y 100 caracteres."
    ))]
    pub name: String,

    #[validate(range(
        min = 0.01,
        max = 1.0,
        message = "El porcentaje de la evaluación debe estar entre 1 y 100%."
    ))]
    pub weight: f32,
}

impl TryFrom<CreateAsignatureDto> for CreateAsignatureInput {
    type Error = AsignatureError;
    fn try_from(dto: CreateAsignatureDto) -> Result<Self, Self::Error> {
        Ok(CreateAsignatureInput {
            year: dto.year,
            code: dto.code,
            name: dto.name,
            evaluations: dto.evaluations.into_iter().map(|e| e.into()).collect(),
            teacher_id: dto
                .teacher_id
                .parse()
                .map_err(|_| AsignatureError::InvalidIdentifier)?,
        })
    }
}

impl From<EvaluationDto> for Evaluation {
    fn from(dto: EvaluationDto) -> Self {
        Evaluation {
            id: Uuid::new_v4(),
            name: dto.name,
            weight: dto.weight,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateAsignatureDto {
    #[validate(range(
        min = 2000,
        max = 2100,
        message = "El año debe tener 4 dígitos."
    ))]
    pub year: Option<i16>,

    #[validate(
        length(equal = 7, message = "El código debe tener 7 caracteres."),
        regex(
            path = *ASIGNATURE_CODE_REGEX,
            message = "El código debe seguir el formato INFO{NNNN}."
        )
    )]
    pub code: Option<String>,

    #[validate(length(
        min = 1,
        max = 100,
        message = "El nombre debe tener entre 1 y 100 caracteres."
    ))]
    pub name: Option<String>,

    #[validate(nested)]
    pub evaluations: Option<Vec<EvaluationDto>>,

    #[validate(custom(function = validate_uuid))]
    pub teacher_id: Option<String>,
}

impl From<UpdateAsignatureDto> for UpdateAsignatureInput {
    fn from(dto: UpdateAsignatureDto) -> Self {
        UpdateAsignatureInput {
            year: dto.year,
            code: dto.code,
            name: dto.name,
            evaluations: dto
                .evaluations
                .map(|evs| evs.into_iter().map(Evaluation::from).collect()),
            teacher_id: dto.teacher_id.and_then(|id| id.parse().ok()),
        }
    }
}

mod validators {
    use regex::Regex;
    use std::sync::LazyLock;
    use validator::ValidationError;

    use crate::asignatures::infrastructure::dtos::EvaluationDto;

    /// The asignature code must match the pattern
    /// "INFO{NNNN}" where "NNNNN" is a 4-digit number.
    pub static ASIGNATURE_CODE_REGEX: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^INFO\d{4}$").unwrap());

    pub fn validate_evaluation_weights(
        evaluations: &Vec<EvaluationDto>,
    ) -> Result<(), ValidationError> {
        let mut total_weight = 0.0;

        for evaluation in evaluations {
            total_weight += evaluation.weight;
        }

        if total_weight != 1.0 {
            return Err(ValidationError::new(
                "El porcentaje de la evaluación debe estar entre 1 y 100%.",
            ));
        }

        Ok(())
    }
}
