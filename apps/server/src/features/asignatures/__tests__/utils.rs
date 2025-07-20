use rand::random_range;
use uuid::Uuid;

pub fn generate_unique_code() -> String {
    format!("INFO{:04}", random_range(1000..9999))
}

pub fn generate_unique_name() -> String {
    format!(
        "Test Asignature {}",
        Uuid::new_v4().to_string()[0..8].to_uppercase()
    )
}
