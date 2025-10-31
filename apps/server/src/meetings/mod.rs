mod controllers;
mod dtos {
    mod meeting;
    mod meeting_req;

    pub use meeting::*;
    pub use meeting_req::*;
}

mod entities {
    mod meeting;
    mod meeting_req;

    pub use meeting::*;
    pub use meeting_req::*;
}

mod repositories {
    mod meeting;
    mod meeting_req;

    pub use meeting::*;
    pub use meeting_req::*;
}

mod services {
    mod calendar;
    mod meeting;
    mod meeting_req;

    pub use calendar::*;
    pub use meeting::*;
    pub use meeting_req::*;
}

pub use controllers::*;
pub use dtos::*;
pub use entities::*;
pub use repositories::*;
pub use services::*;

use crate::config::GoogleCalendarConfig;
use sword::prelude::*;

pub struct MeetingsModule;

impl Module for MeetingsModule {
    type Controller = MeetingsController;

    async fn register_providers(
        config: &Config,
        container: &mut DependencyContainer,
    ) {
        let gc_config = config.get::<GoogleCalendarConfig>().unwrap();
        let calendar_service = CalendarService::new(&gc_config).await;

        container.register_provider::<CalendarService>(calendar_service);
    }

    fn register_components(container: &mut DependencyContainer) {
        container.register_component::<MeetingService>();
        container.register_component::<MeetingRepository>();
        container.register_component::<MeetingRequestsRepository>();
        container.register_component::<MeetingRequestService>();
    }
}
