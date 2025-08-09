use std::sync::Arc;
use tokio::sync::{Mutex, mpsc::Receiver};

use crate::{broker::Event, mailer::Mailer, printer::Printer};

pub struct SubscriberServices {
    pub mailer: Arc<Mailer>,
    pub printer: Arc<Printer>,
}

pub struct BrokerSubscriber {
    receiver: Arc<Mutex<Receiver<Event>>>,
    mailer: Arc<Mailer>,
    printer: Arc<Printer>,
}

impl BrokerSubscriber {
    pub fn new(rcv: Receiver<Event>, services: SubscriberServices) -> Self {
        Self {
            receiver: Arc::new(Mutex::new(rcv)),
            mailer: services.mailer,
            printer: services.printer,
        }
    }

    pub async fn subscribe(&self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(event) = self.receiver.lock().await.recv().await {
            match event {
                Event::PracticeApproved => {
                    println!("Practice approved event received.");
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    println!("Practice approved event done!.");
                }
                Event::PracticeCreated => {}
                _ => {
                    eprintln!("Unhandled event: {:?}", event);
                }
            }
        }

        Ok(())
    }
}
