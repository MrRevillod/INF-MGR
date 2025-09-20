use validator::ValidationError;

pub fn validate_uuid(uuid: &str) -> Result<(), ValidationError> {
    if uuid.is_empty() {
        return Err(ValidationError::new(
            "La identificación no puede estar vacía.",
        ));
    }

    if uuid::Uuid::parse_str(uuid).is_err() {
        return Err(ValidationError::new("Identificación inválida."));
    }

    Ok(())
}
