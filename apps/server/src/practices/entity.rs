use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Practice {
    pub id: Uuid,
    pub enterprise_name: String,
    pub location: String,
    pub description: String,

    pub supervisor_name: String,
    pub supervisor_email: String,
    pub supervisor_phone: String,

    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,

    pub is_approved: bool,
}
