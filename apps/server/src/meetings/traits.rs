use crate::{meetings::*, shared::AppResult, types::*, users::User};

// ------------- Repository Trait Definitions

#[async_trait]
pub trait MeetingRequestsRepository: Interface {
    async fn find_many(
        &self,
        filter: MeetingRequestFilter,
    ) -> AppResult<Vec<MeetingRequest>>;

    async fn find_by_id(&self, id: &Uuid) -> AppResult<Option<MeetingRequest>>;

    async fn create(&self, meeting_req: MeetingRequest)
    -> AppResult<MeetingRequest>;
}

#[async_trait]
pub trait MeetingRepository: Interface {
    async fn find_many(&self, filter: MeetingFilter) -> AppResult<Vec<Meeting>>;
    async fn save(&self, meeting: &Meeting) -> AppResult<Meeting>;
}

// ------------- Service Trait Definitions

#[async_trait]
pub trait MeetingRequestService: Interface {
    async fn get_all(
        &self,
        filter: MeetingRequestFilter,
    ) -> AppResult<Vec<MeetingRequest>>;

    async fn create(
        &self,
        input: CreateMeetingRequestDto,
    ) -> AppResult<MeetingRequest>;

    async fn can_schedule(
        &self,
        user: &User,
        meeting_req_id: &Uuid,
    ) -> AppResult<bool>;
}

#[async_trait]
pub trait MeetingService: Interface {
    async fn get_all(&self, filter: MeetingFilter) -> AppResult<Vec<Meeting>>;
    async fn schedule(
        &self,
        meeting_req_id: &Uuid,
        input: ScheduleMeetingDto,
    ) -> AppResult<Meeting>;
}
