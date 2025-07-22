use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::practices::domain::Practice;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct PracticeModel {
    pub id: uuid::Uuid,
    #[sqlx(rename = "enterprise_name")]
    pub enterprise_name: String,
    pub location: String,
    pub description: String,
    #[sqlx(rename = "supervisor_name")]
    pub supervisor_name: String,
    #[sqlx(rename = "supervisor_email")]
    pub supervisor_email: String,
    #[sqlx(rename = "start_date")]
    pub start_date: DateTime<Utc>,
    #[sqlx(rename = "end_date")]
    pub end_date: DateTime<Utc>,
}

impl From<PracticeModel> for Practice {
    fn from(model: PracticeModel) -> Self {
        Practice {
            id: model.id,
            enterprise_name: model.enterprise_name,
            location: model.location,
            description: model.description,
            supervisor_name: model.supervisor_name,
            supervisor_email: model.supervisor_email,
            start_date: model.start_date,
            end_date: model.end_date,
        }
    }
}

impl From<Practice> for PracticeModel {
    fn from(practice: Practice) -> Self {
        PracticeModel {
            id: practice.id,
            enterprise_name: practice.enterprise_name,
            location: practice.location,
            description: practice.description,
            supervisor_name: practice.supervisor_name,
            supervisor_email: practice.supervisor_email,
            start_date: practice.start_date,
            end_date: practice.end_date,
        }
    }
}
