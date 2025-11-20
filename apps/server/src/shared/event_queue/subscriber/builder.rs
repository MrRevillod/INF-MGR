use crate::{
    config::EventQueueConfig,
    shared::event_queue::{Event, EventSubscriber, SubscriberHandler},
};

use services::{
    config::ServicesConfig, embeddings::EmbeddingService, mailer::Mailer,
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

        let qdrant_service = EmbeddingService::new(services_config.embedding_service)
            .await
            .expect("Failed to create embedding service");

        EventSubscriber {
            config: event_queue_config,
            receiver: Arc::new(Mutex::new(rx)),
            handler: Arc::new(SubscriberHandler {
                printer,
                mailer,
                embedding_service: qdrant_service,
            }),
        }
    }
}
