use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::Type, FromRow};
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
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

impl User {
    pub fn is_student(&self) -> bool {
        self.roles.contains(&Role::Student)
    }

    pub fn is_teacher(&self) -> bool {
        self.roles.contains(&Role::Teacher)
    }

    pub fn is_coordinator(&self) -> bool {
        self.roles.contains(&Role::Coordinator)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, PartialEq)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum Role {
    Administrator,
    Student,
    Teacher,
    Secretary,
    Coordinator,
}
