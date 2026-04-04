use redis::AsyncCommands;
use std::sync::Arc;
use sword::core::injectable;

use crate::{
    auth::Session,
    shared::{AppError, AppResult, RedisDatabase},
};

#[injectable]
pub struct SessionRepository {
    redis_db: Arc<RedisDatabase>,
}

impl SessionRepository {
    pub async fn get(&self, session_id: &str) -> AppResult<Option<Session>> {
        let mut client = self.redis_db.get_connection();
        let key = format!("session:{session_id}");

        let data: Option<String> = client.get(&key).await?;

        match data {
            Some(json_str) => {
                let session: Session = serde_json::from_str(&json_str)
                    .map_err(|e| AppError::InternalServerError(e.into()))?;
                Ok(Some(session))
            }
            None => Ok(None),
        }
    }

    pub async fn save(&self, session: Session, ttl: u64) -> AppResult<()> {
        let mut client = self.redis_db.get_connection();
        let data = serde_json::to_string(&session)
            .map_err(|e| AppError::InternalServerError(e.into()))?;

        let key = format!("session:{}", session.id);

        let _: () = client.set_ex(&key, data, ttl).await?;

        Ok(())
    }

    pub async fn delete(&self, session_id: &str) -> AppResult<()> {
        let mut client = self.redis_db.get_connection();
        let key = format!("session:{session_id}");
        let _: () = client.del(key).await?;

        Ok(())
    }
}
