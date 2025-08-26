use axum::http::StatusCode;
use axum_test::{
    TestServer,
    multipart::{MultipartForm, Part},
};
use serde_json::{Value, json};
use sword::web::ResponseBody;

use crate::{TEST_EMAILS, extract_resource_id, users::utils::TestUser};

pub struct TestPractice {}

impl TestPractice {
    pub fn builder() -> PracticeBuilder {
        PracticeBuilder::new()
    }

    pub async fn extract_id(practice: &Value) -> String {
        extract_resource_id(practice)
    }

    pub async fn create_without_extract(
        app: &TestServer,
        enrollment_id: &String,
        data: Value,
    ) -> Value {
        let route = format!("/enrollments/{}/practice", enrollment_id);
        let response = app.post(&route).json(&data).await;
        let body = response.json::<ResponseBody>();

        assert_eq!(response.status_code(), 201, "Failed to create practice: {}", body.data);

        body.data
    }

    pub async fn create(app: &TestServer, enrollment_id: &String, data: Value) -> String {
        let route = format!("/enrollments/{}/practice", enrollment_id);
        let response = app.post(&route).json(&data).await;
        let body = response.json::<ResponseBody>();

        assert_eq!(response.status_code(), 201, "Failed to create practice: {}", body.data);

        extract_resource_id(&body.data)
    }

    pub async fn approve(app: &TestServer, enrollment_id: &String, practice_id: &String) {
        let route = format!("/enrollments/{}/practice/{}/approve", enrollment_id, practice_id);
        let response = app.post(&route).await;

        assert_eq!(response.status_code(), 200, "Failed to approve practice");
    }

    pub async fn authorize(app: &TestServer, enrollment_id: &String, practice_id: &String) {
        let route = format!("/enrollments/{}/practice/{}/authorize", enrollment_id, practice_id);
        let pdf_part =
            Part::bytes(include_bytes!("../../files/Autorización de práctica.pdf").as_slice())
                .file_name(&"auth_doc.pdf")
                .mime_type(&"text/pdf");

        let form = MultipartForm::new().add_part("auth_doc", pdf_part);
        let response = app.post(&route).multipart(form).await;

        assert_eq!(response.status_code(), 200, "Failed to authorize practice");
    }

    pub async fn delete(app: &TestServer, practice_id: &str) {
        app.delete(&format!("/enrollments/practice/{}", practice_id))
            .await
            .assert_status(StatusCode::NO_CONTENT);
    }
}

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
            supervisor_email: TEST_EMAILS.get("supervisor").map(|s| s.to_string()),
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
            "supervisorEmail": self.supervisor_email.unwrap_or(TestUser::generate_unique_email()),
            "supervisorPhone": self.supervisor_phone,
            "startDate": self.start_date,
            "endDate": self.end_date
        })
    }
}
