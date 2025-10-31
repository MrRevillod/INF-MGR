use redis::{Client, aio::MultiplexedConnection};
use sword::core::injectable;

use crate::config::RedisConfig;

#[injectable(provider)]
pub struct RedisDatabase {
    pub connection: MultiplexedConnection,
}

impl RedisDatabase {
    pub async fn new(config: &RedisConfig) -> Result<Self, redis::RedisError> {
        let client = Client::open(config.url.clone())?;
        let connection = client.get_multiplexed_async_connection().await?;

        Ok(Self { connection })
    }

    pub fn get_connection(&self) -> MultiplexedConnection {
        self.connection.clone()
    }
}
