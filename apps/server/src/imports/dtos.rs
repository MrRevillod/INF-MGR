use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::users::validate_rut_id;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ImportedStudent {
    #[validate(custom(function = "validate_rut_id"))]
    pub rut: String,

    #[validate(length(
        min = 5,
        max = 100,
        message = "El nombre debe tener entre 5 y 100 caracteres."
    ))]
    pub name: String,

    #[validate(email(message = "El email debe ser válido."))]
    pub email: String,

    #[validate(length(
        min = 1,
        max = 100,
        message = "El número de matrícula debe tener entre 1 y 100 caracteres."
    ))]
    pub register: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct ImportStudentsRequest {
    #[validate(nested)]
    pub students: Vec<ImportedStudent>,
}
