use std::{
    fmt::{self, Display},
    str::FromStr,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::users::domain::{User, UserError};

// The `UserModel` struct represents the user model in the database.
// Implements the `FromRow` trait from the `sqlx` crate.
// Implements the `Serialize` and `Deserialize` traits from the `serde` crate.

// Serialize: UserModel -> JSON
// Deserialize: JSON | OTHERS -> UserModel

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserModel {
    pub id: Uuid,
    pub rut: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub roles: Vec<Role>,
    #[sqlx(rename = "deleted_at")]
    pub deleted_at: Option<DateTime<Utc>>,
    #[sqlx(rename = "created_at")]
    pub created_at: DateTime<Utc>,
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

impl Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let role_str = match self {
            Role::Administrator => "administrator",
            Role::Student => "student",
            Role::Teacher => "teacher",
            Role::Secretary => "secretary",
            Role::Coordinator => "coordinator",
        };

        write!(f, "{role_str}")
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
            _ => Err(UserError::InvalidRole {
                role: s.to_string(),
            }),
        }
    }
}

/// Converts a Vec<String> to Vec<Role>, ignoring invalid roles.
pub fn vec_string_to_roles(roles: Vec<impl Into<String>>) -> Vec<Role> {
    roles
        .into_iter()
        .filter_map(|role| Role::from_str(&role.into()).ok())
        .collect()
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
            roles: user_model
                .roles
                .iter()
                .map(|role| role.to_string())
                .collect(),
            deleted_at: user_model.deleted_at,
            created_at: user_model.created_at,
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
            roles: user
                .roles
                .iter()
                .map(|role| Role::from_str(role).unwrap_or(Role::Student))
                .collect(),
            deleted_at: user.deleted_at,
            created_at: user.created_at,
        }
    }
}
