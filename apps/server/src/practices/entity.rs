use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use sqlx::{FromRow, Type};

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

    pub practice_status: PracticeStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq)]
#[sqlx(type_name = "practice_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum PracticeStatus {
    Pending,
    Approved,
    Declined,
}
