use crate::{config::EventQueueConfig, shared::services::*};

use std::sync::Arc;
use tokio::sync::{Mutex, mpsc::Receiver};

#[derive(Default)]
pub struct EventSubscriberBuilder {
    receiver: Option<Receiver<Event>>,
    mailer: Option<Mailer>,
    printer: Option<Printer>,
    config: Option<EventQueueConfig>,
}

impl EventSubscriberBuilder {
    pub fn new() -> Self {
        Self {
            receiver: None,
            mailer: None,
            printer: None,
            config: None,
        }
    }

    pub fn with_receiver(mut self, rx: Receiver<Event>) -> Self {
        self.receiver = Some(rx);
        self
    }

    pub fn with_mailer(mut self, mailer: Mailer) -> Self {
        self.mailer = Some(mailer);
        self
    }

    pub fn with_printer(mut self, printer: Printer) -> Self {
        self.printer = Some(printer);
        self
    }

    pub fn with_config(mut self, config: EventQueueConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn build(self) -> EventSubscriber {
        let rx = self.receiver.expect("Receiver is required");
        let mailer = self.mailer.expect("Mailer is required");
        let printer = self.printer.expect("Printer is required");
        let config = self.config.expect("Config is required");

        let handler = Arc::new(SubscriberHandler { mailer, printer });

        EventSubscriber {
            config,
            receiver: Arc::new(Mutex::new(rx)),
            handler,
        }
    }
}
