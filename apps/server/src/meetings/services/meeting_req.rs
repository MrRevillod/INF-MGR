use crate::{
    courses::*,
    meetings::*,
    types::*,
    users::{Role, UserRepository},
};

use crate::shared::{
    errors::*,
    services::{Event, EventQueue},
};

#[derive(Component)]
#[shaku(interface =  MeetingRequestService)]
pub struct MeetingRequestServiceImpl {
    #[shaku(inject)]
    meeting_reqs: Arc<dyn MeetingRequestsRepository>,

    #[shaku(inject)]
    users: Arc<dyn UserRepository>,

    #[shaku(inject)]
    event_queue: Arc<dyn EventQueue>,

    #[shaku(inject)]
    courses: Arc<dyn CourseRepository>,
}

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
}

#[async_trait]
impl MeetingRequestService for MeetingRequestServiceImpl {
    async fn get_all(
        &self,
        filter: MeetingRequestFilter,
    ) -> AppResult<Vec<MeetingRequest>> {
        self.meeting_reqs.find_many(filter).await
    }

    async fn create(
        &self,
        input: CreateMeetingRequestDto,
    ) -> AppResult<MeetingRequest> {
        let meeting_req = MeetingRequest::from(input);

        let mut teacher = None;
        let mut students = vec![];

        if meeting_req.attendees.len() < 2 {
            return Err(ValidationError::NotEnoughAttendees)?;
        }

        for attendee in &meeting_req.attendees {
            let Some(user) = self.users.find_by_id(attendee).await? else {
                return Err(NotFoundError::user(*attendee))?;
            };

            match user.role {
                Role::Teacher => teacher = Some(user),
                Role::Student => students.push(user),
                _ => continue,
            }
        }

        if teacher.is_none() || students.is_empty() {
            return Err(ValidationError::NotEnoughAttendees)?;
        }

        let Some(course) = self.courses.find_by_id(&meeting_req.course_id).await?
        else {
            return Err(NotFoundError::course(meeting_req.course_id))?;
        };

        let meeting_req = self.meeting_reqs.create(meeting_req).await?;

        let event_data = (teacher.unwrap(), course, students);

        self.event_queue
            .publish(Event::MeetingRequestCreated(event_data))
            .await;

        Ok(meeting_req)
    }
}
