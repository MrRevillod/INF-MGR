use async_trait::async_trait;
use redis::{aio::MultiplexedConnection, Client};
use shaku::{Component, Interface};

use crate::config::RedisConfig;

#[derive(Component)]
#[shaku(interface = CacheDbConnection)]
pub struct RedisDatabase {
    pub connection: MultiplexedConnection,
}

#[async_trait]
pub trait CacheDbConnection: Interface {
    fn get_connection(&self) -> MultiplexedConnection;
}

impl RedisDatabase {
    pub async fn new(config: &RedisConfig) -> Result<Self, redis::RedisError> {
        let client = Client::open(config.url.clone())?;
        let connection = client.get_multiplexed_async_connection().await?;
        Ok(Self { connection })
    }
}

#[async_trait]
impl CacheDbConnection for RedisDatabase {
    fn get_connection(&self) -> MultiplexedConnection {
        self.connection.clone()
    }
}

impl From<RedisDatabase> for RedisDatabaseParameters {
    fn from(db: RedisDatabase) -> Self {
        RedisDatabaseParameters {
            connection: db.connection,
        }
    }
}
