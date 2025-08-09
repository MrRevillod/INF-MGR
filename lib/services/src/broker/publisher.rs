use async_trait::async_trait;
use shaku::{Component, Interface};
use std::{error::Error, sync::Arc};

use crate::broker::{Event, EventSender};

#[derive(Component)]
#[shaku(interface = EventQueue)]
pub struct TokioEventQueue {
    #[shaku(inject)]
    sender: Arc<dyn EventSender>,
}

#[async_trait]
pub trait EventQueue: Interface {
    async fn publish(&self, event: Event) -> Result<(), Box<dyn Error>>;
}

#[async_trait]
impl EventQueue for TokioEventQueue {
    async fn publish(&self, event: Event) -> Result<(), Box<dyn Error>> {
        self.sender.send(event).await?;

        Ok(())
    }
}
