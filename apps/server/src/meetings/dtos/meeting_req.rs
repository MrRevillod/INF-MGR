use crate::{
    meetings::{MeetingRequest, MeetingRequestFilter, MeetingStatus},
    shared::{validate_uuid, validate_uuids},
    types::*,
};

#[derive(Debug, Clone, Validate, Serialize, Deserialize, Default)]
pub struct GetMeetingRequestsQueryDto {
    #[validate(custom(function = validate_uuid))]
    pub student_id: Option<String>,
    #[validate(custom(function = validate_uuid))]
    pub teacher_id: Option<String>,
}

impl From<GetMeetingRequestsQueryDto> for MeetingRequestFilter {
    fn from(dto: GetMeetingRequestsQueryDto) -> Self {
        MeetingRequestFilter {
            student_id: dto.student_id.map(|id| Uuid::parse_str(&id).unwrap()),
            teacher_id: dto.teacher_id.map(|id| Uuid::parse_str(&id).unwrap()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CreateMeetingRequestDto {
    #[validate(custom(function = validate_uuid))]
    teacher_id: String,

    #[validate(custom(function = validate_uuid))]
    course_id: String,

    #[validate(custom(function = validate_uuids))]
    attendees: Vec<String>,
}

impl From<CreateMeetingRequestDto> for MeetingRequest {
    fn from(dto: CreateMeetingRequestDto) -> Self {
        MeetingRequest {
            id: Uuid::new_v4(),
            status: MeetingStatus::Requested,
            course_id: Uuid::parse_str(&dto.course_id).unwrap(),
            attendees: {
                let mut attendees = dto.attendees;

                attendees.push(dto.teacher_id);

                attendees.sort();
                attendees.dedup();

                attendees
                    .into_iter()
                    .map(|id| Uuid::parse_str(&id).unwrap())
                    .collect()
            },

            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
