use crate::{
    auth::JsonWebTokenService,
    shared::{
        database::PostgresDatabase, di::DependencyContainer, oauth::GoogleOAuthClient,
        redis::RedisDatabase, services::event_queue::TokioEventSender,
    },
};

#[derive(Default)]
pub struct DependencyContainerBuilder {
    postgres_db: Option<PostgresDatabase>,
    sender: Option<TokioEventSender>,
    oauth_client: Option<GoogleOAuthClient>,
    redis_db: Option<RedisDatabase>,
    jwt_service: Option<JsonWebTokenService>,
}

impl DependencyContainerBuilder {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_postgres_db(mut self, db: PostgresDatabase) -> Self {
        self.postgres_db = Some(db);
        self
    }

    pub fn with_event_sender(mut self, sender: TokioEventSender) -> Self {
        self.sender = Some(sender);
        self
    }

    pub fn with_oauth_client(mut self, client: GoogleOAuthClient) -> Self {
        self.oauth_client = Some(client);
        self
    }

    pub fn with_redis_db(mut self, db: RedisDatabase) -> Self {
        self.redis_db = Some(db);
        self
    }

    pub fn with_jwt_service(mut self, service: JsonWebTokenService) -> Self {
        self.jwt_service = Some(service);
        self
    }

    pub fn build(self) -> DependencyContainer {
        DependencyContainer::new(
            self.postgres_db.expect("Postgres database is required"),
            self.sender.expect("Event sender is required"),
            self.oauth_client.expect("OAuth client is required"),
            self.redis_db.expect("Redis database is required"),
            self.jwt_service.expect("JWT service is required"),
        )
    }
}
