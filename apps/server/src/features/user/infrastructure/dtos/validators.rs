use regex::Regex;
use validator::ValidationError;

use super::body::{CreateUserDto, UpdateUserDto};

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

    let special_chars_pattern = r#"[!@#$%^&*()_+\-=\[\]{};':"\\|,.<>\/?]"#;
    let special_chars_regex = Regex::new(special_chars_pattern).unwrap();
    let has_special = special_chars_regex.is_match(password);

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
