use async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;

use crate::broker::{Event, EventSender};

#[derive(Component)]
#[shaku(interface = EventQueue)]
pub struct TokioEventQueue {
    #[shaku(inject)]
    sender: Arc<dyn EventSender>,
}

#[async_trait]
pub trait EventQueue: Interface {
    async fn publish(&self, event: Event);
}

#[async_trait]
impl EventQueue for TokioEventQueue {
    async fn publish(&self, event: Event) {
        if let Err(e) = self.sender.send(event).await {
            eprintln!("Error publishing event: {}", e);
        }
    }
}
