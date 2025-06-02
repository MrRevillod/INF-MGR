// This module contains the data transfer objects (DTOs) for user creation and update.

use serde::Deserialize;
use validator::Validate;

use crate::features::user::application::interfaces::{
    CreateUserInput, UpdateUserInput,
};

use super::validators::*;

#[derive(Deserialize, Validate)]
#[validate(schema(function = "validate_password_pairs"))]
pub struct CreateUserDto {
    #[validate(custom(function = "validate_rut_id"))]
    pub id: String,

    #[validate(length(min = 5, max = 100))]
    pub name: String,

    #[validate(email)]
    pub email: String,

    #[validate(custom(function = "password_schema"))]
    pub password: String,

    #[validate(custom(function = "password_schema"))]
    #[serde(rename = "confirmPassword")]
    pub confirm_password: String,

    #[validate(custom(function = "role_validator"))]
    pub roles: Vec<String>,
}

// This trait implementation converts the `CreateUserDto` into the `CreateUserInput`
// This is necessary bc the `CreateUserInput` type is used in the use case
// and the `CreateUserDto` type is used in the controller

// | Controller (CreateUserDto) -> Use Case (CreateUserInput) |

impl From<CreateUserDto> for CreateUserInput {
    fn from(dto: CreateUserDto) -> Self {
        CreateUserInput {
            id: dto.id,
            name: dto.name,
            email: dto.email,
            password: dto.password,
            roles: dto.roles,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
#[validate(schema(function = "validate_optional_password_pairs"))]
pub struct UpdateUserDto {
    #[validate(email)]
    pub email: Option<String>,

    #[validate(custom(function = "password_schema"))]
    pub password: Option<String>,

    #[validate(custom(function = "password_schema"))]
    #[serde(rename = "confirmPassword")]
    pub confirm_password: Option<String>,

    #[validate(custom(function = "role_validator"))]
    pub roles: Option<Vec<String>>,
}

// This trait implementation converts the `UpdateUserDto` into the `UpdateUserInput`
// This is necessary bc the `UpdateUserInput` type is used in the use case
// and the `UpdateUserDto` type is used in the controller

// | Controller (UpdateUserDto) -> Use Case (UpdateUserInput) |

impl From<UpdateUserDto> for UpdateUserInput {
    fn from(dto: UpdateUserDto) -> Self {
        UpdateUserInput {
            email: dto.email,
            password: dto.password,
            roles: dto.roles,
        }
    }
}
