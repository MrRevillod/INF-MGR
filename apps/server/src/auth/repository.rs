use std::sync::Arc;

use async_trait::async_trait;
use redis::AsyncTypedCommands;
use shaku::{Component, Interface};

use crate::shared::{AppError, redis::CacheDbConnection};

#[derive(Component)]
#[shaku(interface = AuthRepository)]
pub struct AuthRepositoryImpl {
    #[shaku(inject)]
    redis_db: Arc<dyn CacheDbConnection>,
}

#[async_trait]
pub trait AuthRepository: Interface {
    async fn save_csrf_token(&self, token: &str) -> Result<(), AppError>;
    async fn get_csrf_token(&self, token: &str) -> Result<Option<String>, AppError>;
}

#[async_trait]
impl AuthRepository for AuthRepositoryImpl {
    async fn save_csrf_token(&self, token: &str) -> Result<(), AppError> {
        self.redis_db
            .get_connection()
            .set_ex(format!("csrf:{token}"), "valid", 600)
            .await?;

        Ok(())
    }

    async fn get_csrf_token(&self, token: &str) -> Result<Option<String>, AppError> {
        let result: Option<String> =
            self.redis_db.get_connection().get(format!("csrf:{token}")).await?;

        Ok(result)
    }
}
