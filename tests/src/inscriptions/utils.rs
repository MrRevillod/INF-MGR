use axum_test::TestServer;
use serde_json::{Value, json};
use sword::web::ResponseBody;

pub struct InscriptionBuilder {
    pub student_id: Option<String>,
    pub asignature_id: Option<String>,
}

impl InscriptionBuilder {
    pub fn new() -> Self {
        Self {
            student_id: None,
            asignature_id: None,
        }
    }

    pub fn with_student_id(mut self, student_id: &String) -> Self {
        self.student_id = Some(student_id.clone());
        self
    }

    pub fn with_asignature_id(mut self, asignature_id: &String) -> Self {
        self.asignature_id = Some(asignature_id.clone());
        self
    }

    pub fn build(self) -> Value {
        json!({
            "studentId": self.student_id,
            "courseId": self.asignature_id
        })
    }
}

pub async fn create_inscription(app: &TestServer, inscription: &Value) -> Value {
    let response = app.post("/inscriptions").json(inscription).await;
    let body = response.json::<ResponseBody>();

    assert_eq!(
        response.status_code(),
        201,
        "Failed to create inscription: {}",
        body.data
    );

    body.data
}

pub async fn delete_incription(app: &TestServer, inscription_id: &str) {
    let response = app
        .delete(&format!("/inscriptions/{}", inscription_id))
        .await;

    assert_eq!(response.status_code(), 200);
}
