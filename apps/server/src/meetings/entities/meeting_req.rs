use crate::types::*;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct MeetingRequest {
    pub id: Uuid,
    pub status: MeetingStatus,
    pub attendees: Vec<Uuid>,
    pub course_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
pub enum MeetingStatus {
    Requested,
    Scheduled,
}

#[derive(Debug, Clone)]
pub struct MeetingRequestFilter {
    pub student_id: Option<Uuid>,
    pub teacher_id: Option<Uuid>,
}

impl Display for MeetingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_str = match self {
            MeetingStatus::Requested => "requested",
            MeetingStatus::Scheduled => "scheduled",
        };

        write!(f, "{}", status_str)
    }
}

impl From<&str> for MeetingStatus {
    fn from(value: &str) -> Self {
        match value {
            "requested" => MeetingStatus::Requested,
            "scheduled" => MeetingStatus::Scheduled,
            _ => MeetingStatus::Requested,
        }
    }
}
