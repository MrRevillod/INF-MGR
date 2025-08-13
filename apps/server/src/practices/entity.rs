use chrono::{DateTime, Utc};
use sea_query::Iden;
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
    IsApproved,
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
            Practices::IsApproved => "is_approved",
        }
    }
}

pub const PRACTICE_INSERT_COLUMNS: [Practices; 10] = [
    Practices::Id,
    Practices::EnterpriseName,
    Practices::Location,
    Practices::Description,
    Practices::SupervisorName,
    Practices::SupervisorEmail,
    Practices::SupervisorPhone,
    Practices::StartDate,
    Practices::EndDate,
    Practices::IsApproved,
];

pub const PRACTICE_UPDATE_COLUMNS: [Practices; 9] = [
    Practices::EnterpriseName,
    Practices::Location,
    Practices::Description,
    Practices::SupervisorName,
    Practices::SupervisorEmail,
    Practices::SupervisorPhone,
    Practices::StartDate,
    Practices::EndDate,
    Practices::IsApproved,
];
