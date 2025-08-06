use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::{practices::Practice, shared::validators::validate_uuid};

#[derive(Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
#[validate(schema(function = "validate_create_practice_dates"))]
pub struct CreatePracticeDto {
    #[validate(length(
        min = 1,
        max = 255,
        message = "El nombre de la debe contener entre 1 y 255 caracteres."
    ))]
    pub enterprise_name: String,

    #[validate(custom(function = validate_uuid, message = "Identificador de curso inválido"))]
    pub enrollment_id: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "La ubicación debe contener entre 1 y 255 caracteres."
    ))]
    pub description: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "El nombre del supervisor debe contener entre 1 y 255 caracteres."
    ))]
    pub location: String,

    #[validate(length(
        min = 1,
        max = 100,
        message = "El nombre del supervisor debe contener entre 1 y 255 caracteres."
    ))]
    pub supervisor_name: String,

    #[validate(email(
        message = "El correo electrónico del supervisor debe ser válido."
    ))]
    pub supervisor_email: String,

    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

impl From<CreatePracticeDto> for Practice {
    fn from(dto: CreatePracticeDto) -> Self {
        Practice {
            id: Uuid::new_v4(),
            enrollment_id: Uuid::parse_str(&dto.enrollment_id).unwrap(),
            enterprise_name: dto.enterprise_name,
            description: dto.description,
            location: dto.location,
            supervisor_name: dto.supervisor_name,
            supervisor_email: dto.supervisor_email,
            start_date: dto.start_date,
            end_date: dto.end_date,
            is_approved: false,
        }
    }
}

#[derive(Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
#[validate(schema(function = "validate_update_practice_dates"))]
pub struct UpdatePracticeDto {
    #[validate(length(
        min = 1,
        max = 255,
        message = "El nombre de la debe contener entre 1 y 255 caracteres."
    ))]
    pub enterprise_name: Option<String>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "La ubicación debe contener entre 1 y 255 caracteres."
    ))]
    pub description: Option<String>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "La ubicación debe contener entre 1 y 255 caracteres."
    ))]
    pub location: Option<String>,

    #[validate(length(
        min = 1,
        max = 100,
        message = "El nombre del supervisor debe contener entre 1 y 255 caracteres."
    ))]
    pub supervisor_name: Option<String>,

    #[validate(email(
        message = "El correo electrónico del supervisor debe ser válido."
    ))]
    pub supervisor_email: Option<String>,

    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

fn validate_create_practice_dates(
    schema: &CreatePracticeDto,
) -> Result<(), ValidationError> {
    validate_dates(schema.start_date, schema.end_date)
}

fn validate_update_practice_dates(
    schema: &UpdatePracticeDto,
) -> Result<(), ValidationError> {
    validate_dates(schema.start_date, schema.end_date)
}

fn validate_dates(
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
) -> Result<(), ValidationError> {
    if start_date.is_none() && end_date.is_some() {
        return Err(ValidationError::new(
            "La fecha de inicio es obligatoria si se proporciona una fecha de finalización.",
        ));
    }

    if start_date.is_some() && end_date.is_none() {
        return Err(ValidationError::new(
            "La fecha de finalización es obligatoria si se proporciona una fecha de inicio.",
        ));
    }

    if let (Some(start), Some(end)) = (start_date, end_date) {
        if start > end {
            return Err(ValidationError::new(
                "La fecha de inicio no puede ser posterior a la fecha de finalización.",
            ));
        }
    }

    Ok(())
}
