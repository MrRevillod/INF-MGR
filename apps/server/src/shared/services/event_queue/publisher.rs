use async_trait::async_trait;
use shaku::{Component, Interface};
use tokio::sync::mpsc::Sender;

use crate::shared::services::event_queue::Event;

#[derive(Component)]
#[shaku(interface = EventQueue)]
pub struct TokioEventQueue {
    sender: Sender<Event>,
}

#[async_trait]
pub trait EventQueue: Interface {
    async fn publish(&self, event: Event);
}

#[async_trait]
impl EventQueue for TokioEventQueue {
    async fn publish(&self, event: Event) {
        if let Err(e) = self.sender.send(event).await {
            tracing::error!("Error publishing event: {e}");
        }
    }
}

impl TokioEventQueue {
    pub fn new(sender: Sender<Event>) -> Self {
        Self { sender }
    }
}

impl From<TokioEventQueue> for TokioEventQueueParameters {
    fn from(queue: TokioEventQueue) -> Self {
        TokioEventQueueParameters {
            sender: queue.sender,
        }
    }
}
