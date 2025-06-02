use chrono::{DateTime, Utc};

use super::UserError;

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub roles: Vec<Role>,
    pub validated: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Role {
    Student,
    Administrator,
    Coordinator,
    Mentor,
    Secretary,
}

impl TryFrom<String> for Role {
    type Error = UserError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "student" => Ok(Role::Student),
            "administrator" => Ok(Role::Administrator),
            "coordinator" => Ok(Role::Coordinator),
            "mentor" => Ok(Role::Mentor),
            "secretary" => Ok(Role::Secretary),
            _ => Err(UserError::InvalidRole),
        }
    }
}

impl From<Role> for String {
    fn from(role: Role) -> Self {
        match role {
            Role::Student => "student".to_string(),
            Role::Administrator => "administrator".to_string(),
            Role::Coordinator => "coordinator".to_string(),
            Role::Mentor => "mentor".to_string(),
            Role::Secretary => "secretary".to_string(),
        }
    }
}
