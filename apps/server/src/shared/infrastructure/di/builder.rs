use crate::{
    auth::JsonWebTokenService,
    shared::{services::TokioEventQueue, *},
};

#[derive(Default)]
pub struct DependencyContainer {
    postgres_db: Option<PostgresDatabase>,
    event_queue: Option<TokioEventQueue>,
    oauth_client: Option<GoogleOAuthClient>,
    redis_db: Option<RedisDatabase>,
    jwt_service: Option<JsonWebTokenService>,
}

impl DependencyContainer {
    pub fn builder() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn with_postgres_db(mut self, db: PostgresDatabase) -> Self {
        self.postgres_db = Some(db);
        self
    }

    pub fn with_event_queue(mut self, event_queue: TokioEventQueue) -> Self {
        self.event_queue = Some(event_queue);
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

    pub fn build(self) -> AppModule {
        let postgres_db = self.postgres_db.expect("PostgresDatabase is required");
        let event_queue = self.event_queue.expect("TokioEventQueue is required");
        let oauth_client = self.oauth_client.expect("GoogleOAuthClient is required");

        let redis_db = self.redis_db.expect("RedisDatabase is required");
        let jwt_service = self.jwt_service.expect("JsonWebTokenService is required");

        AppModule::builder()
            .with_component_parameters::<PostgresDatabase>(postgres_db.into())
            .with_component_parameters::<TokioEventQueue>(event_queue.into())
            .with_component_parameters::<GoogleOAuthClient>(oauth_client.into())
            .with_component_parameters::<RedisDatabase>(redis_db.into())
            .with_component_parameters::<JsonWebTokenService>(jwt_service.into())
            .build()
    }
}
