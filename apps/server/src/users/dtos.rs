use chrono::Utc;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::OnceLock};
use uuid::Uuid;
use validator::{Validate, ValidationError};

use crate::{
    shared::errors::{AppError, Input},
    users::{Role, User, UserFilter},
};

// ============================================================================
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>> CREATE USER DTO <<<<<<<<<<<<<<<<<<<<<<<<<<<
// ============================================================================

#[derive(Serialize, Deserialize, Validate, Debug, Clone)]
#[validate(schema(function = "validate_password_pairs"))]
pub struct CreateUserDto {
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

    #[validate(custom(function = "password_schema"))]
    pub password: String,

    #[validate(custom(function = "password_schema"))]
    #[serde(rename = "confirmPassword")]
    pub confirm_password: String,

    #[validate(custom(function = "role_validator"))]
    pub roles: Vec<String>,
}

impl TryFrom<CreateUserDto> for User {
    type Error = AppError;

    fn try_from(dto: CreateUserDto) -> Result<Self, Self::Error> {
        let roles = from_string_vec_roles(dto.roles)?;

        Ok(User {
            id: Uuid::new_v4(),
            rut: dto.rut,
            name: dto.name,
            email: dto.email,
            password: dto.password,
            roles,
            deleted_at: None,
            created_at: Utc::now(),
        })
    }
}

impl FromStr for Role {
    type Err = AppError;

    fn from_str(role: &str) -> Result<Self, Self::Err> {
        match role.to_lowercase().as_str() {
            "administrator" => Ok(Role::Administrator),
            "teacher" => Ok(Role::Teacher),
            "student" => Ok(Role::Student),
            "secretary" => Ok(Role::Secretary),
            _ => Err(AppError::InvalidInput(Input {
                field: "role".to_string(),
                message: "Rol inválido".to_string(),
                value: role.to_string(),
            })),
        }
    }
}

pub fn from_string_vec_roles(roles: Vec<String>) -> Result<Vec<Role>, AppError> {
    roles
        .into_iter()
        .map(|role| Role::from_str(&role))
        .collect::<Result<Vec<Role>, AppError>>()
}

// ============================================================================
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>> UPDATE USER DTO <<<<<<<<<<<<<<<<<<<<<<<<<<<
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Validate)]
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

// ============================================================================
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>> GET USERS QUERY <<<<<<<<<<<<<<<<<<<<<<<<<<<
// ============================================================================

#[derive(Serialize, Deserialize, Validate)]
pub struct GetUsersQueryDto {
    #[validate(length(
        min = 1,
        max = 100,
        message = "El término de búsqueda debe tener entre 1 y 100 caracteres."
    ))]
    pub search: Option<String>,

    #[validate(range(min = 1, message = "La página debe ser mayor o igual a 1."))]
    pub page: Option<usize>,
}

impl From<GetUsersQueryDto> for UserFilter {
    fn from(dto: GetUsersQueryDto) -> Self {
        UserFilter {
            search: dto.search,
            page: dto.page.unwrap_or(1) as i64,
        }
    }
}

// ============================================================================
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>> USER RESPONSE <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
// ============================================================================

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub rut: String,
    pub name: String,
    pub email: String,
    pub roles: Vec<Role>,
    pub created_at: String,
}

impl From<User> for UserResponse {
    fn from(user_model: User) -> Self {
        UserResponse {
            id: user_model.id.to_string(),
            rut: user_model.rut,
            name: user_model.name,
            email: user_model.email,
            roles: user_model.roles.clone(),
            created_at: user_model.created_at.to_rfc3339(),
        }
    }
}

// ============================================================================
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>> VALIDATORS <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
// ============================================================================

static SPECIAL_CHAR_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_special_char_regex() -> &'static Regex {
    SPECIAL_CHAR_REGEX.get_or_init(|| {
        Regex::new(r#"[!@#$%^&*()_+\-=\[\]{};':"\\|,.<>\/?]"#).unwrap()
    })
}

fn validate_password_pairs(dto: &CreateUserDto) -> Result<(), ValidationError> {
    if dto.password != dto.confirm_password {
        return Err(ValidationError::new("Passwords must match"));
    }
    Ok(())
}

fn validate_optional_password_pairs(
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

fn password_schema(password: &str) -> Result<(), ValidationError> {
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

/// Valida si el RUT chileno es válido
/// Formato esperado: "12345678-5" (con guion y dígito verificador)
fn validate_rut_id(rut: &str) -> Result<(), ValidationError> {
    let Some((number_part, dv_part)) = rut.split_once('-') else {
        return Err(ValidationError::new("invalid_rut_format"));
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

/// Calcula el dígito verificador (DV) de un RUT chileno
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

/// Valida que la lista de roles sea válida
fn role_validator(roles: &Vec<String>) -> Result<(), ValidationError> {
    if roles.is_empty() {
        return Err(ValidationError::new(
            "La lista de roles no pueden estar vacía",
        ));
    }

    for role in roles {
        if role.is_empty() {
            return Err(ValidationError::new("Rol invalido: no puede estar vacío"));
        }

        match Role::from_str(role) {
            Ok(_) => continue,
            Err(_) => return Err(ValidationError::new("Rol invalido")),
        }
    }

    Ok(())
}
