use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::{
    meetings::{Meeting, MeetingStatus},
    shared::validate_uuid,
};

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct CreateMeetingDto {
    #[validate(custom(function = validate_uuid))]
    pub teacher_id: String,

    #[validate(custom(function = validate_uuid))]
    pub student_id: String,
}

impl From<CreateMeetingDto> for Meeting {
    fn from(value: CreateMeetingDto) -> Self {
        Meeting {
            id: Uuid::new_v4(),
            teacher_id: value.teacher_id.parse().unwrap(),
            student_id: value.student_id.parse().unwrap(),
            created_at: Utc::now(),
            start_date: None,
            status: MeetingStatus::from("pending"),
        }
    }
}

// ============================================================================
// >>>>>>>>>>>>>>>>>>>>>>>>>>>>>> MEETING RESPONSE <<<<<<<<<<<<<<<<<<<<<<<<<<<<
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeetingResponse {
    pub id: String,
    pub teacher_id: String,
    pub student_id: String,
    pub start_date: Option<String>,
    pub status: MeetingStatus,
    pub created_at: String,
}

impl From<Meeting> for MeetingResponse {
    fn from(m: Meeting) -> Self {
        MeetingResponse {
            id: m.id.to_string(),
            teacher_id: m.teacher_id.to_string(),
            student_id: m.student_id.to_string(),
            start_date: m.start_date.map(|d| d.to_rfc3339()),
            status: m.status,
            created_at: m.created_at.to_rfc3339(),
        }
    }
}
