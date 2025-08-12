use chrono::{DateTime, Utc};
use chrono_tz::America::Santiago;
use serde::de::{DeserializeOwned, Error};
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum Event {
    PracticeCreated(Value),
    PracticeApproved(Value),
    UserCreated(Value),
}

pub fn get_json<T: DeserializeOwned>(
    data: &Value,
    key: &str,
) -> Result<T, serde_json::Error> {
    if let Some(v) = data.get(key) {
        return serde_json::from_value::<T>(v.clone());
    }
    Err(serde_json::Error::custom("No se encontro la key"))
}

pub fn format_date(date: String) -> String {
    let date = DateTime::parse_from_rfc3339(&date)
        .map(|dt| dt.with_timezone(&Utc))
        .ok();

    date.map(|date| date.with_timezone(&Santiago).format("%d/%m/%y").to_string())
        .unwrap_or_default()
}
