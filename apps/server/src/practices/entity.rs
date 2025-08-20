use chrono::{DateTime, Utc};
use sea_query::Iden;
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

    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,

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

pub enum Practices {
    Table,
    Id,
    EnterpriseName,
    Location,
    Description,
    SupervisorName,
    SupervisorEmail,
    SupervisorPhone,
    StartDate,
    EndDate,
    PracticeStatus,
}

impl Iden for Practices {
    fn unquoted(&self) -> &str {
        match self {
            Practices::Table => "practices",
            Practices::Id => "id",
            Practices::EnterpriseName => "enterprise_name",
            Practices::Location => "location",
            Practices::Description => "description",
            Practices::SupervisorName => "supervisor_name",
            Practices::SupervisorEmail => "supervisor_email",
            Practices::SupervisorPhone => "supervisor_phone",
            Practices::StartDate => "start_date",
            Practices::EndDate => "end_date",
            Practices::PracticeStatus => "practice_status",
        }
    }
}
