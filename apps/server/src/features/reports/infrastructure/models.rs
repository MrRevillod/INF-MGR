use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::reports::domain::Report;

#[derive(Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct ReportModel {
    pub id: Uuid,
    #[sqlx(rename = "student_id")]
    pub student_id: Uuid,
    pub title: String,
    pub content: String,
    #[sqlx(rename = "created_at")]
    pub created_at: DateTime<Utc>,
    #[sqlx(rename = "updated_at")]
    pub updated_at: DateTime<Utc>,
}

impl From<ReportModel> for Report {
    fn from(value: ReportModel) -> Self {
        Report {
            id: value.id,
            student_id: value.student_id,
            title: value.title,
            content: value.content,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<Report> for ReportModel {
    fn from(value: Report) -> Self {
        ReportModel {
            id: value.id,
            student_id: value.student_id,
            title: value.title,
            content: value.content,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
