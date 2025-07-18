use async_trait::async_trait;
use shaku::Interface;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub rut: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Debug)]
pub enum UserError {
    NotFound,
    EmailAlreadyExists,
    InvalidEmail,
    IdAlreadyExists,
    UnexpectedError(String),
    InvalidRole,
}

#[async_trait]
pub trait UserRepository: Interface {
    async fn find_all(&self) -> Result<Vec<User>, UserError>;
    async fn find_by_id(&self, user_id: &Uuid) -> Result<Option<User>, UserError>;
    async fn find_by_rut(&self, rut: &str) -> Result<Option<User>, UserError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError>;
    async fn create(&self, user: User) -> Result<User, UserError>;
    async fn update(&self, user: User) -> Result<User, UserError>;
    async fn delete(&self, user_id: &Uuid) -> Result<(), UserError>;
}
