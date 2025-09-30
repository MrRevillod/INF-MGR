use std::fmt::Display;

use chrono::{DateTime, Utc};
use sea_query::Iden;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Meeting {
    pub id: Uuid,
    pub teacher_id: Uuid,
    pub student_id: Uuid,
    pub start_date: Option<DateTime<Utc>>,
    pub status: MeetingStatus,
    pub created_at: DateTime<Utc>,
}

pub struct MeetingFilter {
    pub teacher_id: Option<Uuid>,
    pub student_id: Option<Uuid>,
    pub status: Option<MeetingStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, Eq)]
pub enum MeetingStatus {
    Pending,
    Confirmed,
    Cancelled,
}

pub enum Meetings {
    Table,
    Id,
    TeacherId,
    StudentId,
    StartDate,
    Status,
    CreatedAt,
}

impl Iden for Meetings {
    fn unquoted(&self) -> &str {
        match self {
            Meetings::Table => "meetings",
            Meetings::Id => "id",
            Meetings::TeacherId => "teacher_id",
            Meetings::StudentId => "student_id",
            Meetings::StartDate => "start_date",
            Meetings::Status => "status",
            Meetings::CreatedAt => "created_at",
        }
    }
}

impl Display for MeetingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_str = match self {
            MeetingStatus::Pending => "pending",
            MeetingStatus::Confirmed => "confirmed",
            MeetingStatus::Cancelled => "cancelled",
        };

        write!(f, "{}", status_str)
    }
}

impl From<&str> for MeetingStatus {
    fn from(value: &str) -> Self {
        match value {
            "pending" => MeetingStatus::Pending,
            "confirmed" => MeetingStatus::Confirmed,
            "cancelled" => MeetingStatus::Cancelled,
            _ => MeetingStatus::Pending,
        }
    }
}
