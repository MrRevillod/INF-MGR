use chrono::{DateTime, Utc};
use chrono_tz::America::Santiago;
use serde::de::{DeserializeOwned, Error};
use serde_json::Value;

use crate::{
    courses::Course, enrollments::Enrollment, practices::Practice, users::User,
};

#[derive(Debug, Clone)]
pub enum Event {
    PracticeCreated((User, Practice, Course, Enrollment)),
    PracticeApproved((User, Practice, Course, User)),
    UserCreated((String, String, String)),
}

pub fn get_json<T: DeserializeOwned>(
    data: &Value,
    key: &str,
) -> Result<T, serde_json::Error> {
    if let Some(v) = data.get(key) {
        return serde_json::from_value::<T>(v.clone());
    }
    Err(serde_json::Error::custom(format!(
        "{key}-No se encontro la key"
    )))
}

pub fn format_date(date: String) -> String {
    let date = DateTime::parse_from_rfc3339(&date)
        .map(|dt| dt.with_timezone(&Utc))
        .ok();

    date.map(|date| date.with_timezone(&Santiago).format("%d/%m/%y").to_string())
        .unwrap_or_default()
}
