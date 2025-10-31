use crate::{config::EventQueueConfig, shared::event_handler::*};

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

    pub fn build(self) -> EventSubscriber {
        let rx = self.receiver.expect("Receiver is required");
        let config = self.config.expect("Config is required");
        let mailer_config = config
            .get::<MailerConfig>()
            .expect("Failed to load mailer config");

        let template_config = config
            .get::<TemplateConfig>()
            .expect("Failed to load template config");

        let printer =
            Printer::new(&template_config).expect("Failed to create printer");

        let mailer = Mailer::new(&mailer_config, &template_config)
            .expect("Failed to create mailer");

        let handler = Arc::new(SubscriberHandler { printer, mailer });

        EventSubscriber {
            config: config
                .get::<EventQueueConfig>()
                .expect("Failed to load event queue config"),
            receiver: Arc::new(Mutex::new(rx)),
            handler,
        }
    }
}
