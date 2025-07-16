use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::features::user::domain::{User, UserError};

// The `UserModel` struct represents the user model in the database.
// Implements the `FromRow` trait from the `sqlx` crate.
// Implements the `Serialize` and `Deserialize` traits from the `serde` crate.

// Serialize: UserModel -> JSON
// Deserialize: JSON | OTHERS -> UserModel

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct UserModel {
    pub id: Uuid,
    pub rut: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: Role,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum Role {
    Administrator,
    Student,
    Teacher,
    Secretary,
    Coordinator,
}

impl ToString for Role {
    fn to_string(&self) -> String {
        match self {
            Role::Administrator => "administrator".to_string(),
            Role::Student => "student".to_string(),
            Role::Teacher => "teacher".to_string(),
            Role::Secretary => "secretary".to_string(),
            Role::Coordinator => "coordinator".to_string(),
        }
    }
}

impl FromStr for Role {
    type Err = UserError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "administrator" => Ok(Role::Administrator),
            "student" => Ok(Role::Student),
            "teacher" => Ok(Role::Teacher),
            "secretary" => Ok(Role::Secretary),
            "coordinator" => Ok(Role::Coordinator),
            _ => Err(UserError::InvalidRole),
        }
    }
}

// This is the conversion from the `UserModel` struct to the `User` struct.
// This is necessary bc the `UserModel` struct is used in the infrastructure layer,
// while the `User` struct is used in the application and domain layers.

// Basically, the repository impl has to return the an User entity,
// but the database returns a UserModel struct.

impl From<UserModel> for User {
    fn from(user_model: UserModel) -> Self {
        User {
            id: user_model.id,
            rut: user_model.rut,
            name: user_model.name,
            email: user_model.email,
            password: user_model.password,
            role: user_model.role.to_string(),
        }
    }
}

// This is the conversion from the `User` struct to the `UserModel` struct.
// It's the opposite of the previous conversion.

// The use case impl returns a User entity, but the controller
// needs to return a UserModel struct to the client.

impl From<User> for UserModel {
    fn from(user: User) -> Self {
        UserModel {
            id: user.id,
            rut: user.rut,
            name: user.name,
            email: user.email,
            password: user.password,
            role: user.role.parse().unwrap_or(Role::Student),
        }
    }
}
