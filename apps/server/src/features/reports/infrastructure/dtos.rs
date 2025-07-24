use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::reports::application::UpdateReportInput;
use crate::reports::domain::ReportFilter;
use crate::shared::validators::validate_uuid;

#[derive(Serialize, Deserialize, Validate)]
pub struct UpdateReportDto {
    #[validate(length(min = 1, max = 255))]
    pub title: Option<String>,

    #[validate(length(min = 1))]
    pub content: Option<String>,
}

impl From<UpdateReportDto> for UpdateReportInput {
    fn from(dto: UpdateReportDto) -> Self {
        UpdateReportInput {
            title: dto.title,
            content: dto.content,
        }
    }
}

#[derive(Serialize, Deserialize, Validate, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetReportsQuery {
    #[validate(custom(function = "validate_uuid"))]
    pub inscription_id: Option<String>,
}

impl From<GetReportsQuery> for ReportFilter {
    fn from(query: GetReportsQuery) -> Self {
        ReportFilter {
            inscription_id: query
                .inscription_id
                .map(|id| Uuid::parse_str(&id).unwrap()),
        }
    }
}
