use redis::AsyncTypedCommands;
use std::sync::Arc;
use sword::core::injectable;

use crate::shared::{AppResult, RedisDatabase};

#[injectable]
pub struct OAuthRepository {
    redis_db: Arc<RedisDatabase>,
}

impl OAuthRepository {
    pub async fn save_csrf_token(&self, token: &str) -> AppResult<()> {
        self.redis_db
            .get_connection()
            .set_ex(format!("csrf:{token}"), "valid", 600)
            .await?;

        Ok(())
    }

    pub async fn get_csrf_token(&self, token: &str) -> AppResult<Option<String>> {
        let result: Option<String> = self
            .redis_db
            .get_connection()
            .get(format!("csrf:{token}"))
            .await?;

        Ok(result)
    }
}
