use sword::core::injectable;
use tokio::sync::mpsc::Sender;

use super::Event;

#[injectable(provider)]
pub struct EventQueue {
    sender: Sender<Event>,
}

impl EventQueue {
    pub const fn new(sender: Sender<Event>) -> Self {
        Self { sender }
    }

    pub async fn publish(&self, event: Event) {
        if let Err(e) = self.sender.send(event).await {
            tracing::error!("Error publishing event: {e}");
        }
    }
}
