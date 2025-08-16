use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::shared::validators::validate_uuid;
use crate::users::{validate_rut_id, User};

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
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
}

#[derive(Debug, Serialize, Deserialize, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ImportCourseDto {
    #[validate(nested)]
    pub students: Vec<ImportUserDto>,

    #[validate(custom(function = validate_uuid))]
    pub id: String,
}

/// This struct is a temporal dto to map over the raw data
/// and preserve plain password to use in related Events
pub struct ImportedUser {
    pub entity: User,
    pub plain_password: String,
}
