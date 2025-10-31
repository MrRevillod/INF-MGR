use crate::{meetings::MeetingFilter, shared::validate_uuid, types::*};

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct ScheduleMeetingDto {
    #[validate(custom(function = validate_rfc3339))]
    pub start_date: String,
    #[validate(custom(function = validate_rfc3339))]
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

pub fn validate_rfc3339(date_str: &str) -> Result<(), validator::ValidationError> {
    match DateTime::parse_from_rfc3339(date_str) {
        Ok(dt) => {
            let _utc: DateTime<Utc> = dt.with_timezone(&Utc);
            Ok(())
        }
        Err(_) => Err(validator::ValidationError::new("invalid_datetime")),
    }
}
