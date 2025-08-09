use std::sync::Arc;
use tokio::sync::{Mutex, mpsc::Receiver};

use crate::{broker::Event, mailer::Mailer, printer::Printer};

pub struct SubscriberServices {
    pub mailer: Mailer,
    pub printer: Printer,
}

pub struct EventSubscriber {
    receiver: Arc<Mutex<Receiver<Event>>>,
    mailer: Arc<Mailer>,
    printer: Arc<Printer>,
}

impl EventSubscriber {
    pub fn new(rcv: Receiver<Event>, services: SubscriberServices) -> Self {
        Self {
            receiver: Arc::new(Mutex::new(rcv)),
            mailer: Arc::new(services.mailer),
            printer: Arc::new(services.printer),
        }
    }

    pub async fn subscribe(&self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(event) = self.receiver.lock().await.recv().await {
            let mailer = Arc::clone(&self.mailer);
            let printer = Arc::clone(&self.printer);

            tokio::spawn(async move {
                if let Err(e) = Self::handle_event(event, mailer, printer).await {
                    eprintln!("Error processing event: {}", e);
                }
            });
        }

        Ok(())
    }

    async fn handle_event(
        event: Event,
        _: Arc<Mailer>,
        _: Arc<Printer>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            _ => {
                println!("Received event: {:?}", event);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

                println!("Event processed: {:?}", event);
            }
        }
        Ok(())
    }
}
