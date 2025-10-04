use crate::{
    config::{ConfigService, GoogleCalendarConfig},
    meetings::*,
    shared::{AppError, AppResult, NotFoundError, services::CalendarService},
    types::*,
    users::UserRepository,
};

#[derive(Component)]
#[shaku(interface = MeetingService)]
pub struct MeetingServiceImpl {
    #[shaku(inject)]
    meetings: Arc<dyn MeetingRepository>,

    #[shaku(inject)]
    meeting_reqs: Arc<dyn MeetingRequestsRepository>,

    #[shaku(inject)]
    calendar: Arc<dyn CalendarService>,

    #[shaku(inject)]
    config: Arc<dyn ConfigService>,

    #[shaku(inject)]
    users: Arc<dyn UserRepository>,
}

#[async_trait]
impl MeetingService for MeetingServiceImpl {
    async fn get_all(&self, filter: MeetingFilter) -> AppResult<Vec<Meeting>> {
        self.meetings.find_many(filter).await
    }

    async fn schedule(
        &self,
        meeting_req_id: &Uuid,
        input: ScheduleMeetingDto,
    ) -> AppResult<Meeting> {
        let Some(meeting_req) = self.meeting_reqs.find_by_id(meeting_req_id).await?
        else {
            return Err(NotFoundError::meeting_request(*meeting_req_id))?;
        };

        let start_date = DateTime::parse_from_rfc3339(&input.start_date)
            .map(|dt| dt.with_timezone(&Utc))
            .unwrap();

        let end_date = match &input.end_date {
            Some(date_str) => DateTime::parse_from_rfc3339(date_str)
                .map(|dt| dt.with_timezone(&Utc))
                .unwrap(),
            None => start_date + Duration::minutes(30),
        };

        let meeting = Meeting {
            id: Uuid::new_v4(),
            start_date,
            end_date,
            attendees: meeting_req.attendees,
            summary: Some(String::from("Reunión de inscripción de práctica")),
            ..Default::default()
        };

        let attendees_emails = {
            let mut emails = vec![];

            for attendee_id in meeting.attendees.iter() {
                let Some(user) = self.users.find_by_id(attendee_id).await? else {
                    continue;
                };

                emails.push(user.email);
            }

            emails
        };

        let calendar_id = &self
            .config
            .inner()
            .get::<GoogleCalendarConfig>()
            .unwrap_or_default()
            .calendar_id;

        let event = Event {
            summary: meeting.summary.clone(),
            description: meeting.description.clone(),
            start: Some(EventDateTime {
                date_time: Some(start_date.into()),
                time_zone: Some("America/Santiago".into()),
                ..Default::default()
            }),
            end: Some(EventDateTime {
                date_time: Some(end_date.into()),
                time_zone: Some("America/Santiago".into()),
                ..Default::default()
            }),
            attendees: Some(
                attendees_emails
                    .into_iter()
                    .map(|email| EventAttendee {
                        email: Some(email),
                        ..Default::default()
                    })
                    .collect(),
            ),
            ..Default::default()
        };

        let calendar_result = self
            .calendar
            .hub()
            .events()
            .insert(event, calendar_id)
            .doit()
            .await
            .map_err(|e| AppError::InternalServerError(e.into()))?;

        let created_meeting = Meeting {
            google_event_id: calendar_result.1.id,
            ..meeting
        };

        self.meetings.save(&created_meeting).await?;

        Ok(created_meeting)
    }
}
