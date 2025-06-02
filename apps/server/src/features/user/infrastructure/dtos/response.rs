use crate::features::user::domain::User;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponseDTO {
    pub id: String,
    pub name: String,
    pub email: String,
    pub validated: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<User> for UserResponseDTO {
    fn from(user_model: User) -> Self {
        UserResponseDTO {
            id: user_model.id,
            name: user_model.name,
            email: user_model.email,
            validated: user_model.validated,
            created_at: user_model.created_at,
            updated_at: user_model.updated_at,
        }
    }
}
