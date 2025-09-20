#[cfg(test)]
pub mod courses;

#[cfg(test)]
pub mod enrollments;

#[cfg(test)]
pub mod practices;

#[cfg(test)]
pub mod users;

#[cfg(test)]
pub mod app;

use serde_json::Value;
use std::{collections::HashMap, sync::LazyLock};

pub static TEST_EMAILS: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
    use std::env;

    let mut m = HashMap::new();

    if let Ok(student_email) = env::var("TEST_STUDENT_EMAIL") {
        m.insert("student".to_string(), student_email);
    }

    if let Ok(teacher_email) = env::var("TEST_TEACHER_EMAIL") {
        m.insert("teacher".to_string(), teacher_email);
    }

    if let Ok(supervisor_email) = env::var("TEST_SUPERVISOR_EMAIL") {
        m.insert("supervisor".to_string(), supervisor_email);
    }

    m
});

pub fn extract_resource_id(data: &Value) -> String {
    data.get("id")
        .and_then(|id| id.as_str())
        .map(String::from)
        .unwrap_or_else(|| panic!("Response does not contain 'id': {data:?}"))
}
