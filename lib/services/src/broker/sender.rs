use async_trait::async_trait;
use shaku::{Component, Interface};
use tokio::sync::mpsc::Sender;

use crate::broker::Event;

#[derive(Component)]
#[shaku(interface = EventSender)]
pub struct TokioEventSender {
    sender: Sender<Event>,
}

#[async_trait]
pub trait EventSender: Interface {
    async fn send(&self, event: Event) -> Result<(), Box<dyn std::error::Error>>;
}

impl TokioEventSender {
    pub fn new(sender: Sender<Event>) -> Self {
        Self { sender }
    }
}

#[async_trait]
impl EventSender for TokioEventSender {
    async fn send(&self, event: Event) -> Result<(), Box<dyn std::error::Error>> {
        self.sender.send(event).await?;

        Ok(())
    }
}

impl From<TokioEventSender> for TokioEventSenderParameters {
    fn from(sender: TokioEventSender) -> Self {
        TokioEventSenderParameters {
            sender: sender.sender,
        }
    }
}
