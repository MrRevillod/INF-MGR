use chrono::{DateTime, Utc};
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
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
    pub deleted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn is_student(&self) -> bool {
        self.roles.contains(&Role::Student)
    }

    pub fn is_teacher(&self) -> bool {
        self.roles.contains(&Role::Teacher)
    }

    pub fn is_administrator(&self) -> bool {
        self.roles.contains(&Role::Administrator)
    }

    pub fn is_secretary(&self) -> bool {
        self.roles.contains(&Role::Secretary)
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
}

#[allow(dead_code)]
pub enum Users {
    Table,
    Id,
    Rut,
    Name,
    Email,
    Password,
    Roles,
    CreatedAt,
    DeletedAt,
}

impl Iden for Users {
    fn unquoted(&self) -> &str {
        match self {
            Users::Table => "users",
            Users::Id => "id",
            Users::Rut => "rut",
            Users::Name => "name",
            Users::Email => "email",
            Users::Password => "password",
            Users::Roles => "roles",
            Users::CreatedAt => "created_at",
            Users::DeletedAt => "deleted_at",
        }
    }
}
