use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use validators::ASIGNATURE_CODE_REGEX;

use crate::asignatures::{
    application::UpdateAsignatureInput,
    domain::{Asignature, AsignatureError, Evaluation},
    infrastructure::dtos::validators::{
        validate_evaluation_weights, validate_repeated_evaluation_names,
    },
};

use crate::shared::validators::validate_uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateAsignatureDto {
    #[validate(range(
        min = 2000,
        max = 2100,
        message = "El año debe tener 4 dígitos."
    ))]
    pub year: i32,

    #[validate(
        length(equal = 8, message = "El código debe tener 8 caracteres."),
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
        custom(function = validate_evaluation_weights),
        custom(function = validate_repeated_evaluation_names)
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
    pub weight: f64,
}

impl TryFrom<CreateAsignatureDto> for Asignature {
    type Error = AsignatureError;
    fn try_from(dto: CreateAsignatureDto) -> Result<Self, Self::Error> {
        Ok(Asignature {
            id: Uuid::new_v4(),
            year: dto.year,
            code: dto.code,
            name: dto.name,
            evaluations: dto.evaluations.into_iter().map(|e| e.into()).collect(),
            teacher_id: dto.teacher_id.parse().unwrap(),
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
#[serde(rename_all = "camelCase")]
pub struct UpdateAsignatureDto {
    #[validate(range(
        min = 2000,
        max = 2100,
        message = "El año debe tener 4 dígitos."
    ))]
    pub year: Option<i32>,

    #[validate(
        length(equal = 8, message = "El código debe tener 8 caracteres."),
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

    #[validate(nested, custom(function = validate_repeated_evaluation_names))]
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

        if total_weight * 100_f64 != 100_f64 {
            return Err(ValidationError::new(
                "El porcentaje de la evaluación debe estar entre 1 y 100%.",
            ));
        }

        Ok(())
    }

    pub fn validate_repeated_evaluation_names(
        evaluations: &Vec<EvaluationDto>,
    ) -> Result<(), ValidationError> {
        let mut names = std::collections::HashSet::new();
        for evaluation in evaluations {
            if !names.insert(evaluation.name.clone()) {
                return Err(ValidationError::new(
                    "Los nombres de las evaluaciones deben ser únicos.",
                ));
            }
        }
        Ok(())
    }
}
