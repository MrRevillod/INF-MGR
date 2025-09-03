use async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;

use crate::{
    auth::Session,
    shared::{AppResult, redis::CacheDbConnection},
};

#[derive(Component)]
#[shaku(interface = SessionRepository)]
pub struct RedisSessionRepository {
    #[shaku(inject)]
    _redis_db: Arc<dyn CacheDbConnection>,
}

#[async_trait]
pub trait SessionRepository: Interface {
    async fn get(&self, key: &str) -> AppResult<Option<Session>>;
    async fn save(&self, session: Session) -> AppResult<()>;
}

#[async_trait]
impl SessionRepository for RedisSessionRepository {
    async fn get(&self, _key: &str) -> AppResult<Option<Session>> {
        unimplemented!()
    }

    async fn save(&self, _session: Session) -> AppResult<()> {
        unimplemented!()
    }
}
