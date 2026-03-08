use services::ServiceResult;
use std::sync::Arc;
use tokio::{
    sync::{Mutex, mpsc::Receiver},
    time::{Duration, sleep},
};

mod builder;
mod events;
mod handlers;

pub use builder::*;
pub use events::*;
pub use handlers::*;

use crate::config::EventQueueConfig;

pub struct EventSubscriber {
    receiver: Arc<Mutex<Receiver<Event>>>,
    handler: Arc<SubscriberHandler>,
    config: EventQueueConfig,
}

impl EventSubscriber {
    pub fn builder() -> EventSubscriberBuilder {
        EventSubscriberBuilder::new()
    }

    pub fn run(self) {
        tokio::spawn(async move {
            if let Err(e) = self.subscribe().await {
                tracing::error!("Error in event subscriber: {e}");
            }
        });
    }

    pub async fn subscribe(&self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(event) = self.receiver.lock().await.recv().await {
            let delay_between_event_retry_ms = self.config.delay_between_event_retry_ms;

            let handler = Arc::clone(&self.handler);

            tokio::spawn(async move {
                let handle = Self::handle(Arc::clone(&handler), event.clone()).await;

                if handle.is_err() {
                    tracing::error!(
                        "Error handling event {:?}: {:?}",
                        event,
                        handle.err().unwrap()
                    );
                }

                sleep(Duration::from_secs(delay_between_event_retry_ms)).await;
            });
        }

        Ok(())
    }

    async fn handle(handler: Arc<SubscriberHandler>, event: Event) -> ServiceResult<()> {
        match event {
            Event::CourseCreated(event) => handler.course_created(event).await?,
            Event::UserCreated(event) => handler.user_created(event).await?,
            Event::PracticeCreated(event) => handler.practice_created(event).await,
            Event::PracticeDeclined(event) => handler.practice_declined(event).await,
            Event::PracticeApproved(event) => {
                handler.practice_approved(event).await?;
            }
            Event::PracticeAuthorized(event) => {
                handler.practice_authorized(event).await?;
            }
            Event::PracticeEvaluated(event) => {
                handler.practice_evaluated(event).await;
            }
            Event::FinalReportUploaded(event) => {
                handler.final_report_uploaded(event).await?;
            }
            Event::ImportedStudents(event) => {
                handler.imported_students(event).await?;
            }
            Event::MeetingRequestCreated(event) => {
                handler.meeting_request_created(event).await?;
            }
        }

        Ok(())
    }
}
