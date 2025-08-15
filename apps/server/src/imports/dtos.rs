use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::shared::validators::validate_uuid;
use crate::users::{role_validator, validate_rut_id};

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct ImportUserDto {
    #[validate(custom(function = validate_rut_id))]
    pub rut: String,

    #[validate(length(
        min = 5,
        max = 100,
        message = "El nombre debe tener entre 5 y 100 caracteres."
    ))]
    pub name: String,

    #[validate(email(message = "El email debe ser válido."))]
    pub email: String,

    #[validate(custom(function = role_validator))]
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ImportCourseDataDto {
    #[validate(nested)]
    pub users: Vec<ImportUserDto>,

    #[validate(custom(function = validate_uuid))]
    pub course_id: String,
}
