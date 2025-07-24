use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::users::{application::UpdateUserInput, domain::User};

#[derive(Deserialize, Validate, Debug)]
#[validate(schema(function = "validators::validate_password_pairs"))]
pub struct CreateUserDto {
    #[validate(custom(function = "validators::validate_rut_id"))]
    pub rut: String,

    #[validate(length(
        min = 5,
        max = 100,
        message = "El nombre debe tener entre 5 y 100 caracteres."
    ))]
    pub name: String,

    #[validate(email(message = "El email debe ser válido."))]
    pub email: String,

    #[validate(custom(function = "validators::password_schema"))]
    pub password: String,

    #[validate(custom(function = "validators::password_schema"))]
    #[serde(rename = "confirmPassword")]
    pub confirm_password: String,

    #[validate(custom(function = "validators::role_validator"))]
    pub roles: Vec<String>,
}

// This trait implementation converts the `CreateUserDto` into the `CreateUserInput`
// This is necessary bc the `CreateUserInput` type is used in the use case
// and the `CreateUserDto` type is used in the controller

// | Controller (CreateUserDto) -> Use Case (CreateUserInput) |

impl From<CreateUserDto> for User {
    fn from(dto: CreateUserDto) -> Self {
        User {
            id: Uuid::new_v4(),
            rut: dto.rut,
            name: dto.name,
            email: dto.email,
            password: dto.password,
            roles: dto.roles,
            deleted_at: None,
        }
    }
}

#[derive(Debug, Deserialize, Validate)]
#[validate(schema(function = "validators::validate_optional_password_pairs"))]
pub struct UpdateUserDto {
    #[validate(email)]
    pub email: Option<String>,

    #[validate(custom(function = "validators::password_schema"))]
    pub password: Option<String>,

    #[validate(custom(function = "validators::password_schema"))]
    #[serde(rename = "confirmPassword")]
    pub confirm_password: Option<String>,

    #[validate(custom(function = "validators::role_validator"))]
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

// Response DTOs are used to format the data returned by the API
// They are not used in the use case, but rather in the controller to format the response
// For example, the use case returns a `User` model, but the controller formats it into a `UserResponseDTO`
// taking care of the serialization and response structure and removing any sensitive data like passwords.

#[derive(Serialize)]
pub struct UserResponseDTO {
    pub id: String,
    pub rut: String,
    pub name: String,
    pub email: String,
    pub roles: Vec<String>,
}

impl From<User> for UserResponseDTO {
    fn from(user_model: User) -> Self {
        UserResponseDTO {
            id: user_model.id.to_string(),
            rut: user_model.rut,
            name: user_model.name,
            email: user_model.email,
            roles: user_model.roles.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Validate)]
pub struct GetUsersQuery {
    #[validate(custom(function = "validators::validate_roles_string"))]
    pub role: String,
}

mod validators {
    use regex::Regex;
    use std::{str::FromStr, sync::OnceLock};
    use validator::ValidationError;

    use super::{CreateUserDto, UpdateUserDto};
    use crate::users::infrastructure::models::Role;

    static SPECIAL_CHAR_REGEX: OnceLock<Regex> = OnceLock::new();

    fn get_special_char_regex() -> &'static Regex {
        SPECIAL_CHAR_REGEX.get_or_init(|| {
            Regex::new(r#"[!@#$%^&*()_+\-=\[\]{};':"\\|,.<>\/?]"#).unwrap()
        })
    }

    pub fn validate_password_pairs(
        dto: &CreateUserDto,
    ) -> Result<(), ValidationError> {
        if dto.password != dto.confirm_password {
            return Err(ValidationError::new("Passwords must match"));
        }

        Ok(())
    }

    pub fn validate_optional_password_pairs(
        dto: &UpdateUserDto,
    ) -> Result<(), ValidationError> {
        match (&dto.password, &dto.confirm_password) {
            (Some(pwd), Some(conf)) if pwd != conf => {
                Err(ValidationError::new("Passwords must match"))
            }
            (Some(_), None) | (None, Some(_)) => Err(ValidationError::new(
                "Either provide both password fields or neither",
            )),
            _ => Ok(()),
        }
    }

    pub fn password_schema(password: &str) -> Result<(), ValidationError> {
        if password.len() < 8 || password.len() > 100 {
            return Err(ValidationError::new(
                "Password must be 8-100 characters long",
            ));
        }

        let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
        let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());

        let has_special = get_special_char_regex().is_match(password);

        if !has_uppercase {
            return Err(ValidationError::new(
                "Password must contain at least one uppercase letter",
            ));
        }

        if !has_lowercase {
            return Err(ValidationError::new(
                "Password must contain at least one lowercase letter",
            ));
        }

        if !has_digit {
            return Err(ValidationError::new(
                "Password must contain at least one digit",
            ));
        }

        if !has_special {
            return Err(ValidationError::new(
            "Password must contain at least one special character (e.g., !@#$%^&*)",
        ));
        }

        Ok(())
    }

    /// Valida si el RUT es válido.
    /// Formato esperado: "12345678-5" (con guion y dígito verificador)
    pub fn validate_rut_id(rut: &str) -> Result<(), ValidationError> {
        let Some((number_part, dv_part)) = rut.split_once('-') else {
            return Err(ValidationError::new("invalid_rut_format")); // no contiene guion
        };

        let number: u32 = match number_part.parse() {
            Ok(n) => n,
            Err(_) => return Err(ValidationError::new("invalid_rut_number")),
        };

        let expected_dv = compute_rut_dv(number);
        if expected_dv != dv_part.to_uppercase() {
            return Err(ValidationError::new("invalid_rut_dv"));
        }

        Ok(())
    }

    /// Calcula el dígito verificador (DV) de un RUT chileno.
    fn compute_rut_dv(mut rut: u32) -> String {
        let mut sum = 0;
        let mut multiplier = 2;

        while rut > 0 {
            let digit = rut % 10;
            sum += digit * multiplier;
            rut /= 10;
            multiplier = if multiplier == 7 { 2 } else { multiplier + 1 };
        }

        let remainder = 11 - (sum % 11);
        match remainder {
            11 => "0".to_string(),
            10 => "K".to_string(),
            n => n.to_string(),
        }
    }

    pub fn role_validator(roles: &Vec<String>) -> Result<(), ValidationError> {
        if roles.is_empty() {
            return Err(ValidationError::new(
                "La lista de roles no pueden estar vacía",
            ));
        }

        for role in roles {
            if role.is_empty() {
                return Err(ValidationError::new(
                    "Rol invalido: no puede estar vacío",
                ));
            }

            match Role::from_str(role) {
                Ok(_) => continue,
                Err(_) => return Err(ValidationError::new("Rol invalido")),
            }
        }

        Ok(())
    }

    pub fn validate_roles_string(role: &String) -> Result<(), ValidationError> {
        if role.is_empty() {
            return Err(ValidationError::new("El rol no puede estar vacío"));
        }

        match Role::from_str(role) {
            Ok(_) => Ok(()),
            Err(_) => return Err(ValidationError::new("Rol invalido")),
        }
    }
}
