use crate::{meetings::*, shared::AppResult, types::*};

#[derive(Component)]
#[shaku(interface = MeetingService)]
pub struct MeetingServiceImpl {
    #[shaku(inject)]
    meetings: Arc<dyn MeetingRepository>,
}

#[async_trait]
pub trait MeetingService: Interface {
    async fn get_all(&self, filter: MeetingFilter) -> AppResult<Vec<Meeting>>;

    async fn schedule_meeting(
        &self,
        schdule: ScheduleMeetingDto,
    ) -> AppResult<Meeting>;
}

#[async_trait]
impl MeetingService for MeetingServiceImpl {
    async fn get_all(&self, filter: MeetingFilter) -> AppResult<Vec<Meeting>> {
        self.meetings.find_many(filter).await
    }

    async fn schedule_meeting(&self, _: ScheduleMeetingDto) -> AppResult<Meeting> {
        todo!()
    }
}
