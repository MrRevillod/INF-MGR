use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

pub type Teacher = User;
pub type Student = User;
pub type Secretary = User;
pub type Administrator = User;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: Uuid,
    pub rut: String,
    pub name: String,
    pub email: String,
    pub google_id: Option<String>,
    pub role: Role,
    pub register: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn is_student(&self) -> bool {
        self.role == Role::Student
    }

    pub fn is_teacher(&self) -> bool {
        self.role == Role::Teacher
    }

    pub fn is_administrator(&self) -> bool {
        self.role == Role::Administrator
    }

    pub fn is_secretary(&self) -> bool {
        self.role == Role::Secretary
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Type, Eq, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum Role {
    Administrator,
    Teacher,
    Secretary,
    #[default]
    Student,
}

impl Role {
    pub const fn priority(&self) -> u8 {
        match self {
            Role::Administrator => 4,
            Role::Secretary => 3,
            Role::Teacher => 2,
            Role::Student => 1,
        }
    }
}

impl Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Administrator => "administrator",
                Self::Student => "student",
                Self::Teacher => "teacher",
                Self::Secretary => "secretary",
            }
        )
    }
}

impl Default for User {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            rut: String::default(),
            name: String::default(),
            email: String::default(),
            google_id: None,
            role: Role::default(),
            register: None,
            deleted_at: None,
            created_at: Utc::now(),
        }
    }
}
