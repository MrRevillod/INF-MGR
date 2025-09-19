use async_trait::async_trait;
use redis::AsyncTypedCommands;
use shaku::{Component, Interface};
use std::sync::Arc;

use crate::shared::{AppResult, redis::CacheDbConnection};

#[derive(Component)]
#[shaku(interface = AuthRepository)]
pub struct OAuthRepository {
    #[shaku(inject)]
    redis_db: Arc<dyn CacheDbConnection>,
}

#[async_trait]
pub trait AuthRepository: Interface {
    async fn save_csrf_token(&self, token: &str) -> AppResult<()>;
    async fn get_csrf_token(&self, token: &str) -> AppResult<Option<String>>;
}

#[async_trait]
impl AuthRepository for OAuthRepository {
    async fn save_csrf_token(&self, token: &str) -> AppResult<()> {
        self.redis_db
            .get_connection()
            .set_ex(format!("csrf:{token}"), "valid", 600)
            .await?;

        Ok(())
    }

    async fn get_csrf_token(&self, token: &str) -> AppResult<Option<String>> {
        let result: Option<String> = self
            .redis_db
            .get_connection()
            .get(format!("csrf:{token}"))
            .await?;

        Ok(result)
    }
}
