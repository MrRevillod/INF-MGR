use crate::types::*;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Meeting {
    pub id: Uuid,
    pub google_event_id: Option<String>,
    pub summary: Option<String>,
    pub description: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub attendees: Vec<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct MeetingFilter {
    pub attendee_id: Option<Uuid>,
}

impl Default for Meeting {
    fn default() -> Self {
        Meeting {
            id: Uuid::new_v4(),
            google_event_id: None,
            summary: None,
            description: None,
            start_date: Utc::now(),
            end_date: Utc::now(),
            attendees: vec![],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
