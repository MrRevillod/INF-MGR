use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Practice {
    pub id: Uuid,

    #[sqlx(rename = "enterprise_name")]
    pub enterprise_name: String,
    pub location: String,
    pub description: String,

    #[sqlx(rename = "supervisor_name")]
    pub supervisor_name: String,
    #[sqlx(rename = "supervisor_email")]
    pub supervisor_email: String,

    #[sqlx(rename = "start_date")]
    pub start_date: Option<DateTime<Utc>>,
    #[sqlx(rename = "end_date")]
    pub end_date: Option<DateTime<Utc>>,

    #[sqlx(rename = "is_approved")]
    pub is_approved: bool,
}
