use axum_test::TestServer;
use serde_json::{Value, json};
use sword::web::ResponseBody;

pub struct PracticeBuilder {
    pub enterprise_name: Option<String>,
    pub description: Option<String>,
    pub location: Option<String>,
    pub supervisor_name: Option<String>,
    pub supervisor_email: Option<String>,
    pub supervisor_phone: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

impl PracticeBuilder {
    pub fn new() -> Self {
        Self {
            enterprise_name: None,
            description: None,
            location: None,
            supervisor_name: None,
            supervisor_email: None,
            supervisor_phone: None,
            start_date: None,
            end_date: None,
        }
    }

    pub fn with_enterprise_name(mut self, name: &str) -> Self {
        self.enterprise_name = Some(name.to_string());
        self
    }

    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    pub fn with_location(mut self, location: &str) -> Self {
        self.location = Some(location.to_string());
        self
    }

    pub fn with_supervisor_name(mut self, name: &str) -> Self {
        self.supervisor_name = Some(name.to_string());
        self
    }

    pub fn with_supervisor_email(mut self, email: &str) -> Self {
        self.supervisor_email = Some(email.to_string());
        self
    }

    pub fn with_supervisor_phone(mut self, phone: &str) -> Self {
        self.supervisor_phone = Some(phone.to_string());
        self
    }

    pub fn with_start_date(mut self, date: &str) -> Self {
        self.start_date = Some(date.to_string());
        self
    }

    pub fn with_end_date(mut self, date: &str) -> Self {
        self.end_date = Some(date.to_string());
        self
    }

    pub fn build(self) -> Value {
        json!({
            "enterpriseName": self.enterprise_name,
            "description": self.description,
            "location": self.location,
            "supervisorName": self.supervisor_name,
            "supervisorEmail": self.supervisor_email,
            "supervisorPhone": self.supervisor_phone,
            "startDate": self.start_date,
            "endDate": self.end_date
        })
    }
}
pub async fn create_practice(app: &TestServer, practice: &Value) -> Value {
    let response = app.post("/practices").json(practice).await;
    let body = response.json::<ResponseBody>();

    assert_eq!(response.status_code(), 201, "Failed to create practice: {}", body.data);

    body.data
}
pub async fn delete_practice(app: &TestServer, practice_id: &str) {
    let response = app.delete(&format!("/practices/{}", practice_id)).await;

    assert_eq!(response.status_code(), 200);
}
