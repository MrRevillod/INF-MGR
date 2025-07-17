use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Practice {
    pub id: Uuid,
    pub enterprise_name: String,
    pub location: String,
    pub description: String,
    pub coordinator_name: String,
    pub coordinator_email: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}
