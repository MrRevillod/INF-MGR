use async_trait::async_trait;
use redis::AsyncCommands;
use shaku::{Component, Interface};
use std::sync::Arc;

use crate::{
    auth::Session,
    shared::{AppError, AppResult, redis::CacheDbConnection},
};

#[derive(Component)]
#[shaku(interface = SessionRepository)]
pub struct RedisSessionRepository {
    #[shaku(inject)]
    redis_db: Arc<dyn CacheDbConnection>,
}

#[async_trait]
pub trait SessionRepository: Interface {
    async fn get(&self, key: &str) -> AppResult<Option<Session>>;
    async fn save(&self, session: Session, ttl: u64) -> AppResult<()>;
    async fn delete(&self, session_id: &str) -> AppResult<()>;
}

#[async_trait]
impl SessionRepository for RedisSessionRepository {
    async fn get(&self, session_id: &str) -> AppResult<Option<Session>> {
        let mut client = self.redis_db.get_connection();
        let data: String = client.get(format!("session:{session_id}")).await?;

        let session: Session = serde_json::from_str(&data)
            .map_err(|e| AppError::InternalServerError(e.into()))?;

        Ok(Some(session))
    }

    async fn save(&self, session: Session, ttl: u64) -> AppResult<()> {
        let mut client = self.redis_db.get_connection();
        let data = serde_json::to_string(&session)
            .map_err(|e| AppError::InternalServerError(e.into()))?;

        let key = format!("session:{}", session.id);
        let _: () = client.set_ex(key, data, ttl).await?;

        Ok(())
    }

    async fn delete(&self, session_id: &str) -> AppResult<()> {
        let mut client = self.redis_db.get_connection();
        let key = format!("session:{session_id}");
        let _: () = client.del(key).await?;
        Ok(())
    }
}
