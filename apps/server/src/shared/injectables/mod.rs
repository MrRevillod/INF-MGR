mod oauth;
mod postgres;
mod redis;

pub use oauth::*;
pub use postgres::*;
pub use redis::*;

use sword::prelude::*;

use crate::config::{AuthConfig, PostgresDbConfig, RedisConfig};

pub struct SharedModule;

impl Module for SharedModule {
    type Controller = NonControllerModule;

    async fn register_providers(
        config: &Config,
        container: &mut DependencyContainer,
    ) {
        let auth_config = config.get::<AuthConfig>().unwrap();
        let redis_config = config.get::<RedisConfig>().unwrap();
        let postgres_config = config.get::<PostgresDbConfig>().unwrap();

        container.register_provider(OAuthClient::new(&auth_config));
        container.register_provider(
            PostgresDatabase::new(&postgres_config)
                .await
                .expect("Failed to create PostgresDatabase"),
        );

        container.register_provider(
            RedisDatabase::new(&redis_config)
                .await
                .expect("Failed to create RedisClient"),
        );
    }
}
