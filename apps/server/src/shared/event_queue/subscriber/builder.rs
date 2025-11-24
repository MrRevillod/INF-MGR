use crate::{
    config::EventQueueConfig,
    shared::event_queue::{Event, EventSubscriber, SubscriberHandler},
};

use services::{
    config::ServicesConfig, embeddings::EmbeddingService, mailer::Mailer,
    plagiarism::{PlagiarismDetectionService, PlagiarismRepository},
    printer::Printer,
};

use std::sync::Arc;
use sword::core::Config;
use tokio::sync::{Mutex, mpsc::Receiver};

#[derive(Default)]
pub struct EventSubscriberBuilder {
    receiver: Option<Receiver<Event>>,
    config: Option<Config>,
}

impl EventSubscriberBuilder {
    pub const fn new() -> Self {
        Self {
            receiver: None,
            config: None,
        }
    }

    pub fn with_receiver(mut self, rx: Receiver<Event>) -> Self {
        self.receiver = Some(rx);
        self
    }

    pub fn with_config(mut self, config: Config) -> Self {
        self.config = Some(config);
        self
    }

    pub async fn build(self) -> EventSubscriber {
        let rx = self.receiver.expect("Receiver is required");
        let config = self.config.expect("Config is required");

        let event_queue_config = config
            .get::<EventQueueConfig>()
            .expect("Failed to load event queue config");

        let services_config = config
            .get::<ServicesConfig>()
            .expect("Failed to load services config");

        let mailer = Mailer::new(&services_config).expect("Failed to create mailer");
        let printer = Printer::new(&services_config).expect("Failed to create printer");

        let embedding_service = EmbeddingService::new(services_config.embedding_service)
            .await
            .expect("Failed to create embedding service");

        // Create dedicated database pool for plagiarism service
        let postgres_config = config
            .get::<crate::config::PostgresDbConfig>()
            .expect("Failed to load postgres config");
        
        let db_pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(5) // Small pool for plagiarism service
            .connect(&postgres_config.url)
            .await
            .expect("Failed to create plagiarism database pool");

        // Create plagiarism services
        let plagiarism_repository = PlagiarismRepository::new(db_pool);
        let plagiarism_service = PlagiarismDetectionService::new(
            embedding_service.clone(),
            plagiarism_repository,
        );

        EventSubscriber {
            config: event_queue_config,
            receiver: Arc::new(Mutex::new(rx)),
            handler: Arc::new(SubscriberHandler {
                printer,
                mailer,
                embedding_service,
                plagiarism_service,
            }),
        }
    }
}
