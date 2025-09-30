use async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;

use crate::{
    meetings::{Meeting, MeetingRepository},
    shared::{AppResult, NotFoundError},
    users::UserRepository,
};

#[derive(Component)]
#[shaku(interface = MeetingService)]
pub struct MeetingServiceImpl {
    #[shaku(inject)]
    meetings: Arc<dyn MeetingRepository>,

    #[shaku(inject)]
    users: Arc<dyn UserRepository>,
}

#[async_trait]
pub trait MeetingService: Interface {
    async fn create(
        &self,
        input: crate::meetings::CreateMeetingDto,
    ) -> AppResult<Meeting>;
}

#[async_trait]
impl MeetingService for MeetingServiceImpl {
    async fn create(
        &self,
        input: crate::meetings::CreateMeetingDto,
    ) -> AppResult<Meeting> {
        let meeting = Meeting::from(input);

        let (teacher, student) = tokio::join!(
            self.users.find_by_id(&meeting.teacher_id),
            self.users.find_by_id(&meeting.student_id)
        );

        let Some(_teacher) = teacher? else {
            return Err(NotFoundError::user(meeting.teacher_id))?;
        };
        let Some(_student) = student? else {
            return Err(NotFoundError::user(meeting.student_id))?;
        };

        self.meetings.save(meeting).await
    }
}
