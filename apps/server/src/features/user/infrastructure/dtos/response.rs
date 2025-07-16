use crate::features::user::domain::User;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponseDTO {
    pub id: String,
    pub rut: String,
    pub name: String,
    pub email: String,
    pub role: String,
}

impl From<User> for UserResponseDTO {
    fn from(user_model: User) -> Self {
        UserResponseDTO {
            id: user_model.id.to_string(),
            rut: user_model.rut,
            name: user_model.name,
            email: user_model.email,
            role: user_model.role.to_string(),
        }
    }
}
