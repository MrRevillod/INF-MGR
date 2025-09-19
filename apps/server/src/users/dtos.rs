use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
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

    #[validate(custom(function = "role_validator"))]
    pub role: String,
}

impl TryFrom<CreateUserDto> for User {
    type Error = AppError;

    fn try_from(dto: CreateUserDto) -> Result<Self, Self::Error> {
        let role = Role::from_str(&dto.role)?;

        Ok(User {
            id: Uuid::new_v4(),
            rut: dto.rut,
            name: dto.name,
            email: dto.email,
            google_id: None,
            role,
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

// ============================================================================
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>> UPDATE USER DTO <<<<<<<<<<<<<<<<<<<<<<<<<<<
// ============================================================================

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UpdateUserDto {
    #[validate(email)]
    pub email: Option<String>,

    #[validate(custom(function = "role_validator"))]
    pub role: Option<String>,
}

// ============================================================================
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>> GET USERS QUERY <<<<<<<<<<<<<<<<<<<<<<<<<<<
// ============================================================================

#[derive(Serialize, Deserialize, Validate, Default)]
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
            page: dto.page.unwrap_or(1) as u64,
            ..UserFilter::default()
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
    pub role: Role,
    pub created_at: String,
}

impl From<User> for UserResponse {
    fn from(user_model: User) -> Self {
        UserResponse {
            id: user_model.id.to_string(),
            rut: user_model.rut,
            name: user_model.name,
            email: user_model.email,
            role: user_model.role,
            created_at: user_model.created_at.to_rfc3339(),
        }
    }
}

// ============================================================================
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>> VALIDATORS <<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<
// ============================================================================

/// Valida si el RUT chileno es válido
/// Formato esperado: "12345678-5" (con guion y dígito verificador)
pub fn validate_rut_id(rut: &str) -> Result<(), ValidationError> {
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

/// Valida que el rol sea uno de los valores permitidos
pub fn role_validator(role: &str) -> Result<(), ValidationError> {
    if role.is_empty() {
        return Err(ValidationError::new("El rol no puede estar vacío"));
    }

    if Role::from_str(role).is_err() {
        return Err(ValidationError::new("Rol inválido"));
    }

    Ok(())
}
