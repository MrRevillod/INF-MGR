use crate::{meetings::MeetingFilter, shared::validate_uuid, types::*};

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct ScheduleMeetingDto {
    #[validate(custom(function = validate_uuid))]
    pub meeting_id: String,
    pub start_date: String,
    pub end_date: Option<String>,
}

#[derive(Debug, Clone, Validate, Serialize, Deserialize, Default)]
pub struct GetMeetingsQueryDto {
    #[validate(custom(function = validate_uuid))]
    pub attendee_id: Option<String>,
}

impl From<GetMeetingsQueryDto> for MeetingFilter {
    fn from(dto: GetMeetingsQueryDto) -> Self {
        MeetingFilter {
            attendee_id: dto.attendee_id.map(|id| Uuid::parse_str(&id).unwrap()),
        }
    }
}
