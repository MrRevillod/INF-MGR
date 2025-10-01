use crate::{shared::validate_uuid, types::*};

#[derive(Debug, Serialize, Deserialize, Clone, Validate)]
pub struct ScheduleMeetingDto {
    #[validate(custom(function = validate_uuid))]
    pub meeting_id: String,
    pub start_date: String,
    pub end_date: Option<String>,
}
