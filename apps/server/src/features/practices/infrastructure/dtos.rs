use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::practices::{application::UpdatePracticeInput, domain::Practice};

#[derive(Serialize, Deserialize, Validate)]
#[validate(schema(function = "validators::validate_dates"))]
#[serde(rename_all = "camelCase")]
pub struct CreatePracticeDto {
    #[validate(length(
        min = 1,
        max = 255,
        message = "El nombre de la empresa debe contener entre 1 y 255 caracteres."
    ))]
    pub enterprise_name: String,

    #[validate(length(
        min = 1,
        max = 100,
        message = "La ubicación debe contener entre 1 y 100 caracteres."
    ))]
    pub location: String,

    #[validate(length(
        min = 1,
        max = 500,
        message = "La descripción debe contener entre 1 y 500 caracteres."
    ))]
    pub description: String,

    #[validate(email(
        message = "El correo electrónico del supervisor debe ser válido."
    ))]
    pub supervisor_email: String,

    #[validate(length(
        min = 1,
        max = 100,
        message = "El nombre del supervisor debe contener entre 1 y 100 caracteres."
    ))]
    pub supervisor_name: String,

    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

impl From<CreatePracticeDto> for Practice {
    fn from(dto: CreatePracticeDto) -> Self {
        Practice {
            id: Uuid::new_v4(),
            enterprise_name: dto.enterprise_name,
            location: dto.location,
            description: dto.description,
            supervisor_email: dto.supervisor_email,
            supervisor_name: dto.supervisor_name,
            start_date: dto.start_date,
            end_date: dto.end_date,
        }
    }
}

#[derive(Serialize, Deserialize, Validate)]
#[validate(schema(function = "validators::validate_optional_dates"))]
#[serde(rename_all = "camelCase")]
pub struct UpdatePracticeDto {
    #[validate(length(
        min = 1,
        max = 255,
        message = "El nombre de la empresa debe contener entre 1 y 255 caracteres."
    ))]
    pub enterprise_name: Option<String>,

    #[validate(length(
        min = 1,
        max = 100,
        message = "La ubicación debe contener entre 1 y 100 caracteres."
    ))]
    pub location: Option<String>,

    #[validate(length(
        min = 1,
        max = 500,
        message = "La descripción debe contener entre 1 y 500 caracteres."
    ))]
    pub description: Option<String>,

    #[validate(email(
        message = "El correo electrónico del supervisor debe ser válido."
    ))]
    pub supervisor_email: Option<String>,

    #[validate(length(
        min = 1,
        max = 100,
        message = "El nombre del supervisor debe contener entre 1 y 100 caracteres."
    ))]
    pub supervisor_name: Option<String>,

    #[validate(custom(function = "validators::date_not_in_past"))]
    pub start_date: Option<DateTime<Utc>>,

    #[validate(custom(function = "validators::date_not_in_past"))]
    pub end_date: Option<DateTime<Utc>>,
}

impl From<UpdatePracticeDto> for UpdatePracticeInput {
    fn from(dto: UpdatePracticeDto) -> Self {
        UpdatePracticeInput {
            enterprise_name: dto.enterprise_name,
            location: dto.location,
            description: dto.description,
            supervisor_email: dto.supervisor_email,
            supervisor_name: dto.supervisor_name,
            start_date: dto.start_date,
            end_date: dto.end_date,
        }
    }
}

mod validators {
    use chrono::{DateTime, Utc};
    use validator::ValidationError;

    use crate::practices::infrastructure::dtos::{
        CreatePracticeDto, UpdatePracticeDto,
    };

    pub fn date_not_in_past(date: &DateTime<Utc>) -> Result<(), ValidationError> {
        if *date < Utc::now() {
            return Err(ValidationError::new(
                "La fecha no puede ser anterior a la fecha actual.",
            ));
        }

        Ok(())
    }

    pub fn validate_dates(
        schema: &CreatePracticeDto,
    ) -> Result<(), ValidationError> {
        if schema.end_date < schema.start_date {
            return Err(ValidationError::new(
                "La fecha de finalización no puede ser anterior a la fecha de inicio.",
            ));
        }

        Ok(())
    }

    pub fn validate_optional_dates(
        schema: &UpdatePracticeDto,
    ) -> Result<(), ValidationError> {
        if let (Some(start_date), Some(end_date)) =
            (schema.start_date, schema.end_date)
        {
            if end_date < start_date {
                return Err(ValidationError::new(
                    "La fecha de finalización no puede ser anterior a la fecha de inicio.",
                ));
            }
        }

        Ok(())
    }
}
