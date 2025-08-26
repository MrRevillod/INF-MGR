use axum_test::TestServer;
use chrono::Datelike;
use serde_json::{Value, json};
use sword::web::ResponseBody;
use uuid::Uuid;

use crate::extract_resource_id;

pub struct TestCourse {}

impl TestCourse {
    pub fn builder(teacher_id: &str) -> CourseBuilder {
        CourseBuilder::new(teacher_id)
    }

    pub fn generate_unique_code() -> String {
        let uuid = Uuid::new_v4();
        let uuid_bytes = uuid.as_bytes();
        let numeric_value =
            u32::from_be_bytes([uuid_bytes[0], uuid_bytes[1], uuid_bytes[2], uuid_bytes[3]]);

        let four_digits = (numeric_value % 9000) + 1000;

        format!("INFO{:04}", four_digits)
    }

    pub fn generate_unique_course_name() -> String {
        let uuid = Uuid::new_v4();
        let uuid_str = uuid.to_string().replace("-", "");
        let first_8_chars = &uuid_str[0..8];

        format!("Práctica Test {}", first_8_chars.to_uppercase())
    }

    pub fn extract_id(course: &Value) -> String {
        extract_resource_id(course)
    }

    pub async fn create_without_extract(app: &TestServer, course: &Value) -> ResponseBody {
        let response = app.post("/courses").json(&course).await;
        let body = response.json::<ResponseBody>();

        body
    }

    pub async fn update_should_fail(
        app: &TestServer,
        id: &str,
        course: &Value,
        code: u16,
    ) -> ResponseBody {
        let response = app.patch(&format!("/courses/{}", id)).json(&course).await;
        let body = response.json::<ResponseBody>();

        assert_eq!(body.code, code, "Failed to update course");

        body
    }

    pub async fn create(app: &TestServer, course: &Value) -> Value {
        let response = app.post("/courses").json(&course).await;
        let body = response.json::<ResponseBody>();

        assert_eq!(
            response.status_code(),
            201,
            "{}",
            format!("Failed to create course: {:?}", body.data)
        );

        body.data
    }

    pub async fn update(app: &TestServer, course_id: &str, course: &Value) -> Value {
        let response = app.patch(&format!("/courses/{}", course_id)).json(&course).await;
        let body = response.json::<ResponseBody>();

        assert_eq!(
            response.status_code(),
            200,
            "{}",
            format!("Failed to update course: {:?}", body.data)
        );

        body.data
    }

    pub async fn delete(app: &TestServer, course_id: &str) {
        let response = app.delete(&format!("/courses/{}", course_id)).await;

        assert_eq!(response.status_code(), 200, "Failed to delete course");
    }
}

pub struct CourseBuilder {
    year: i32,
    code: String,
    name: String,
    evaluations: Vec<EvaluationBuilder>,
    teacher_id: String,
}

pub struct EvaluationBuilder {
    name: String,
    weight: i32,
}

impl CourseBuilder {
    pub fn new(teacher_id: &str) -> Self {
        let current_year = chrono::Utc::now().year();
        Self {
            year: current_year,
            code: TestCourse::generate_unique_code(),
            name: TestCourse::generate_unique_course_name(),
            evaluations: vec![
                EvaluationBuilder {
                    name: "Informe Final".to_string(),
                    weight: 60,
                },
                EvaluationBuilder {
                    name: "Evaluación Empresa".to_string(),
                    weight: 40,
                },
            ],
            teacher_id: teacher_id.to_string(),
        }
    }

    pub fn with_year(mut self, year: i32) -> Self {
        self.year = year;
        self
    }

    pub fn with_code(mut self, code: &str) -> Self {
        self.code = code.to_string();
        self
    }

    pub fn with_name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn with_evaluations(mut self, evaluations: Vec<(&str, i32)>) -> Self {
        self.evaluations = evaluations
            .into_iter()
            .map(|(name, weight)| EvaluationBuilder {
                name: name.to_string(),
                weight,
            })
            .collect();
        self
    }

    pub fn with_single_evaluation(mut self, name: &str, weight: i32) -> Self {
        self.evaluations = vec![EvaluationBuilder {
            name: name.to_string(),
            weight,
        }];
        self
    }

    pub fn build(self) -> Value {
        let evaluations = self
            .evaluations
            .into_iter()
            .map(|e| {
                json!({
                    "name": e.name,
                    "weight": e.weight
                })
            })
            .collect::<Vec<_>>();

        json!({
            "year": self.year,
            "code": self.code,
            "name": self.name,
            "evaluations": evaluations,
            "teacherId": self.teacher_id,
        })
    }
}
