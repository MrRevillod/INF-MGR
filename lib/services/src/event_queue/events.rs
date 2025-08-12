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
