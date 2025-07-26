use async_trait::async_trait;
use chrono::{DateTime, Utc};
use shaku::Interface;
use thiserror::Error;
use uuid::Uuid;

use crate::shared::services::errors::ServiceError;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub rut: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub roles: Vec<String>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct GetUsersParams {
    pub roles: Vec<&'static str>, // Comma-separated list of roles
    pub search: Option<String>,   // Search term for name, email, or RUT
}

#[derive(Debug, Error)]
pub enum UserError {
    #[error("User not found")]
    NotFound,

    #[error("User email already exists")]
    EmailAlreadyExists,

    #[error("Invalid email format or domain")]
    InvalidEmail,

    #[error("User RUT already exists: {rut}")]
    RutAlreadyExists { rut: String },

    #[error("User database error: {source}")]
    Database {
        #[from]
        source: sqlx::Error,
    },

    #[error("Service error: {source}")]
    ServiceError {
        #[from]
        source: ServiceError,
    },

    #[error("Invalid user role: {role}")]
    InvalidRole { role: String },
}

#[async_trait]
pub trait UserRepository: Interface {
    async fn find_all(&self, roles: GetUsersParams) -> Result<Vec<User>, UserError>;
    async fn find_by_id(&self, user_id: &Uuid) -> Result<Option<User>, UserError>;
    async fn find_by_rut(&self, rut: &str) -> Result<Option<User>, UserError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError>;
    async fn create(&self, user: User) -> Result<User, UserError>;
    async fn update(&self, user: User) -> Result<User, UserError>;
    async fn delete(&self, user_id: &Uuid) -> Result<(), UserError>;
}
