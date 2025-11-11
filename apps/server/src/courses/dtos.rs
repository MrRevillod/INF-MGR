use chrono::{Datelike, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

use crate::{
    courses::{Course, CourseEvaluation, CourseStatus},
    shared::{AppError, ValidationError as AppValidationError, validate_uuid},
    users::User,
};

// ============================================================================
// >>>>>>>>>>>>>>>>>>>>>>>>>>> CREATE COURSE DTO <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
// ============================================================================

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CreateCourseDto {
    #[validate(custom(function = validate_course_year))]
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
            course_status: CourseStatus::Active,
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
            "active" => Ok(CourseStatus::Active),
            "completed" => Ok(CourseStatus::Completed),
            _ => Err(AppValidationError::invalid_course_status(s.to_string()))?,
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

    #[validate(
        custom(function = validate_course_status)
    )]
    pub status: Option<String>,

    #[validate(
        nested,
        custom(function = validate_update_evaluation_weights)
    )]
    pub evaluations: Option<Vec<UpdateEvaluationDto>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Validate)]
pub struct UpdateEvaluationDto {
    #[validate(custom(function = validate_uuid))]
    pub id: Option<String>,

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

impl From<UpdateEvaluationDto> for CourseEvaluation {
    fn from(dto: UpdateEvaluationDto) -> Self {
        CourseEvaluation {
            id: Uuid::new_v4(),
            name: dto.name,
            weight: dto.weight,
        }
    }
}
// ============================================================================
// >>>>>>>>>>>>>>>>>>>>>>>>>>>> COURSE RESPONSE DTO <<<<<<<<<<<<<<<<<<<<<<<<<<<
// ============================================================================

pub type CourseWithStaff = (Course, User);

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct CourseResponse {
    pub id: Uuid,
    pub year: i32,
    pub code: String,
    pub name: String,
    pub course_status: CourseStatus,
    pub evaluations: Vec<CourseEvaluation>,
    pub teacher_id: Uuid,
    pub teacher: User,
}

impl From<CourseWithStaff> for CourseResponse {
    fn from((course, teacher): CourseWithStaff) -> Self {
        CourseResponse {
            id: course.id,
            year: course.year,
            code: course.code,
            name: course.name,
            evaluations: course.evaluations,
            teacher_id: course.teacher_id,
            course_status: course.course_status,
            teacher,
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

fn validate_weights(weights: &[i32]) -> Result<(), ValidationError> {
    let total: i32 = weights.iter().sum();

    if total != 100 {
        return Err(ValidationError::new("Las evaluaciones deben sumar 100%."));
    }

    Ok(())
}

fn validate_evaluation_weights(
    evaluations: &[CourseEvaluationDto],
) -> Result<(), ValidationError> {
    let weights: Vec<i32> = evaluations.iter().map(|e| e.weight).collect();
    validate_weights(&weights)
}

fn validate_update_evaluation_weights(
    evaluations: &[UpdateEvaluationDto],
) -> Result<(), ValidationError> {
    let weights: Vec<i32> = evaluations.iter().map(|e| e.weight).collect();

    validate_weights(&weights)
}

fn validate_course_status(status: &String) -> Result<(), ValidationError> {
    if status != "active" && status != "completed" {
        return Err(ValidationError::new(
            "El estado de la asignatura debe ser 'active' o 'completed'.",
        ));
    }

    Ok(())
}

fn validate_course_year(year: i32) -> Result<(), ValidationError> {
    let current_year = Utc::now().year();

    if year < current_year || year > current_year + 1 {
        return Err(ValidationError::new(
            "El año de la asignatura debe ser el año actual o el siguiente.",
        ));
    }

    Ok(())
}
