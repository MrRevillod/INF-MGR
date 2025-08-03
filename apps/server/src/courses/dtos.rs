use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use validator::Validate;

use crate::{
    courses::{Course, CourseEvaluation, CourseStatus},
    shared::{
        errors::{AppError, Input},
        validators::validate_uuid,
    },
    users::User,
};

// ============================================================================
// >>>>>>>>>>>>>>>>>>>>>>>>>>> CREATE COURSE DTO <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
// ============================================================================

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateCourseDto {
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
    )]
    pub evaluations: Vec<CourseEvaluationDto>,

    #[validate(custom(function = validate_uuid))]
    pub teacher_id: String,

    #[validate(custom(function = validate_uuid))]
    pub coordinator_id: String,
}

impl From<CreateCourseDto> for Course {
    fn from(dto: CreateCourseDto) -> Self {
        Course {
            id: Uuid::new_v4(),
            year: dto.year,
            code: dto.code,
            name: dto.name,
            evaluations: dto
                .evaluations
                .into_iter()
                .map(CourseEvaluation::from)
                .collect(),

            teacher_id: Uuid::parse_str(&dto.teacher_id).unwrap(),
            coordinator_id: Uuid::parse_str(&dto.coordinator_id).unwrap(),
            status: CourseStatus::InProgress,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct CourseEvaluationDto {
    #[validate(length(
        min = 1,
        max = 100,
        message = "El nombre de la evaluación debe tener entre 1 y 100 caracteres."
    ))]
    pub name: String,

    #[validate(range(
        min = 1,
        max = 100,
        message = "El porcentaje de la evaluación debe estar entre 1 y 100%."
    ))]
    pub weight: i32,
}

impl From<CourseEvaluationDto> for CourseEvaluation {
    fn from(dto: CourseEvaluationDto) -> Self {
        CourseEvaluation {
            id: Uuid::new_v4(),
            name: dto.name,
            weight: dto.weight,
        }
    }
}

impl FromStr for CourseStatus {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inprogress" => Ok(CourseStatus::InProgress),
            "ended" => Ok(CourseStatus::Ended),
            _ => Err(AppError::InvalidInput(Input {
                field: "status".to_string(),
                message: "El estado debe ser 'inprogress' o 'ended'.".to_string(),
                value: s.to_string(),
            })),
        }
    }
}

// ============================================================================
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>> UPDATE COURSE DTO <<<<<<<<<<<<<<<<<<<<<<<<<<<<
// ============================================================================

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UpdateCourseDto {
    #[validate(custom(function = validate_uuid))]
    pub teacher_id: Option<String>,

    #[validate(custom(function = validate_uuid))]
    pub coordinator_id: Option<String>,

    #[validate(
        custom(function = validate_course_status)
    )]
    pub status: Option<String>,
}

// ============================================================================
// >>>>>>>>>>>>>>>>>>>>>>>>>>>> COURSE RESPONSE DTO <<<<<<<<<<<<<<<<<<<<<<<<<<<
// ============================================================================

pub type CourseWithStaff = (Course, User, User);

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CourseResponse {
    pub id: Uuid,
    pub year: i32,
    pub code: String,
    pub name: String,
    pub status: CourseStatus,
    pub evaluations: Vec<CourseEvaluation>,
    pub teacher_id: Uuid,
    pub coordinator_id: Uuid,

    pub teacher: User,
    pub coordinator: User,
}

impl From<CourseWithStaff> for CourseResponse {
    fn from((course, teacher, coord): CourseWithStaff) -> Self {
        CourseResponse {
            id: course.id,
            year: course.year,
            code: course.code,
            name: course.name,
            evaluations: course.evaluations,
            teacher_id: course.teacher_id,
            coordinator_id: course.coordinator_id,
            status: course.status,
            teacher,
            coordinator: coord,
        }
    }
}

// ============================================================================
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> VALIDATORS <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
// ============================================================================

use regex::Regex;
use std::{str::FromStr, sync::LazyLock};
use validator::ValidationError;

static ASIGNATURE_CODE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^INFO\d{4}$").unwrap());

fn validate_evaluation_weights(
    evaluations: &Vec<CourseEvaluationDto>,
) -> Result<(), ValidationError> {
    let mut total_weight = 0;

    for evaluation in evaluations {
        total_weight += evaluation.weight;
    }

    if total_weight != 100 {
        return Err(ValidationError::new("Las evaluaciones deben sumar 100%."));
    }

    Ok(())
}

fn validate_course_status(status: &String) -> Result<(), ValidationError> {
    if status != "inprogress" && status != "ended" {
        return Err(ValidationError::new(
            "El estado de la asignatura debe ser 'inprogress' o 'ended'.",
        ));
    }

    Ok(())
}
