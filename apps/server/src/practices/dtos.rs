use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationError};

use regex::Regex;
use std::sync::LazyLock;

use crate::practices::{entity::PracticeStatus, Practice};

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

    #[validate(email(message = "El correo electrónico del supervisor debe ser válido."))]
    pub supervisor_email: String,

    #[validate(regex(
        path = *PHONE_REGEX,
        message = "El teléfono del supervisor debe ser un número válido."
    ))]
    pub supervisor_phone: String,

    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

impl From<CreatePracticeDto> for Practice {
    fn from(dto: CreatePracticeDto) -> Self {
        Practice {
            id: Uuid::new_v4(),
            enterprise_name: dto.enterprise_name,
            description: dto.description,
            location: dto.location,
            supervisor_name: dto.supervisor_name,
            supervisor_email: dto.supervisor_email,
            supervisor_phone: dto.supervisor_phone,
            start_date: dto.start_date,
            end_date: dto.end_date,
            practice_status: PracticeStatus::Pending,
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

    #[validate(email(message = "El correo electrónico del supervisor debe ser válido."))]
    pub supervisor_email: Option<String>,

    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

static PHONE_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:\+56)?\s?(?:9\d{8}|\d{1}\d{8})$").unwrap());

fn validate_create_practice_dates(schema: &CreatePracticeDto) -> Result<(), ValidationError> {
    validate_dates(Some(schema.start_date), Some(schema.end_date))
}

fn validate_update_practice_dates(schema: &UpdatePracticeDto) -> Result<(), ValidationError> {
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
        if start == end {
            return Err(ValidationError::new(
                "La fecha de inicio no puede ser igual a la fecha de finalización.",
            ));
        }
    }

    Ok(())
}
