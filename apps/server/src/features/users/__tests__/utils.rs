use uuid::Uuid;

pub fn generate_unique_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}
