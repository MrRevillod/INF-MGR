use crate::types::*;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Meeting {
    pub id: Uuid,
    pub google_event_id: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub attendees: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct MeetingFilter {
    pub attendee_id: Option<Uuid>,
}
