use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub validated: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
