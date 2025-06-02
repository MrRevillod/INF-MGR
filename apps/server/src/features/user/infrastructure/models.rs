// This module contains the UserModel struct and its conversions traits.

// |----------------------------------------------------------------|
// |                Return entities between layers                  |
// |----------------------------------------------------------------|
// |  User Infrastructure Layer (UserModel)   | Controller|RepoImpl |
// |------------------------------------------|---------------------|
// |       User Application Layer (User)      |       Use Case      |
// |------------------------------------------|---------------------|
// |         User Domain Layer (User)         |      Repository     |
// |----------------------------------------------------------------|

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, types::Json};

use crate::features::user::domain::{Role, User};

// The `UserModel` struct represents the user model in the database.
// Implements the `FromRow` trait from the `sqlx` crate.
// Implements the `Serialize` and `Deserialize` traits from the `serde` crate.

// Serialize: UserModel -> JSON
// Deserialize: JSON | OTHERS -> UserModel

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct UserModel {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub roles: Json<Vec<String>>,
    pub validated: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
            name: user_model.name,
            email: user_model.email,
            password: user_model.password,
            validated: user_model.validated,
            roles: user_model
                .roles
                .0
                .into_iter()
                .filter_map(|role| Role::try_from(role).ok())
                .collect(),
            created_at: user_model.created_at,
            updated_at: user_model.updated_at,
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
            name: user.name,
            email: user.email,
            password: user.password,
            validated: user.validated,
            roles: Json(user.roles.into_iter().map(String::from).collect()),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
