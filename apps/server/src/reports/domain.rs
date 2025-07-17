use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Report {
    pub id: Uuid,
    pub student_id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
