use axum_test::TestServer;
use serde_json::{Value, json};
use sword::web::ResponseBody;

use crate::extract_resource_id;

pub struct TestEnrollment {}

impl TestEnrollment {
    pub fn builder() -> EnrollmentBuilder {
        EnrollmentBuilder::new()
    }

    pub fn extract_id(enrollment: &Value) -> String {
        extract_resource_id(enrollment)
    }

    pub async fn create(app: &TestServer, enrollment: &Value) -> Value {
        let response = app.post("/courses/enroll").json(enrollment).await;
        let body = response.json::<ResponseBody>();

        assert_eq!(response.status_code(), 201, "Failed to create enrollment: {}", body.data);

        body.data
    }

    pub async fn delete(app: &TestServer, enrollment_id: &str) {
        let response = app.delete(&format!("/courses/enrollments/{}", enrollment_id)).await;

        assert_eq!(response.status_code(), 200);
    }
}

pub struct EnrollmentBuilder {
    pub student_id: Option<String>,
    pub course_id: Option<String>,
}

impl EnrollmentBuilder {
    pub fn new() -> Self {
        Self {
            student_id: None,
            course_id: None,
        }
    }

    pub fn with_student_id(mut self, student_id: &String) -> Self {
        self.student_id = Some(student_id.clone());
        self
    }

    pub fn with_course_id(mut self, course_id: &String) -> Self {
        self.course_id = Some(course_id.clone());
        self
    }

    pub fn build(self) -> Value {
        json!({
            "studentId": self.student_id,
            "courseId": self.course_id
        })
    }
}
