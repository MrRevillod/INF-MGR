use regex::Regex;
use std::{str::FromStr, sync::OnceLock};
use validator::ValidationError;

use super::body::{CreateUserDto, UpdateUserDto};
use crate::features::user::infrastructure::models::Role;

static SPECIAL_CHAR_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_special_char_regex() -> &'static Regex {
    SPECIAL_CHAR_REGEX.get_or_init(|| {
        Regex::new(r#"[!@#$%^&*()_+\-=\[\]{};':"\\|,.<>\/?]"#).unwrap()
    })
}

pub fn validate_password_pairs(dto: &CreateUserDto) -> Result<(), ValidationError> {
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

pub fn role_validator(role: &String) -> Result<(), ValidationError> {
    match Role::from_str(role) {
        Ok(_) => Ok(()),
        Err(_) => Err(ValidationError::new("invalid_role")),
    }
}
