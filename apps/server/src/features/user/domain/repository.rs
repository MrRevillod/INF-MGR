use async_trait::async_trait;
use shaku::Interface;
use uuid::Uuid;

use super::{entity::User, errors::UserError};

#[async_trait]
pub trait UserRepository: Interface {
    async fn find_all(&self) -> Result<Vec<User>, UserError>;
    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, UserError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, UserError>;
    async fn find_by_username(&self, name: &str) -> Result<Option<User>, UserError>;
    async fn create(&self, user: User) -> Result<User, UserError>;
    async fn update(&self, user: User) -> Result<User, UserError>;
    async fn delete(&self, user_id: Uuid) -> Result<(), UserError>;
}
