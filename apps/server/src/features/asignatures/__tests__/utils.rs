use serde_json::{json, Value};
use sword::web::ResponseBody;
use uuid::Uuid;

pub struct AsignatureBuilder {
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

impl AsignatureBuilder {
    pub fn new(teacher_id: &str) -> Self {
        Self {
            year: 2024,
            code: generate_unique_code(),
            name: generate_unique_name(),
            evaluations: vec![
                EvaluationBuilder {
                    name: "Bitácoras Semanales".to_string(),
                    weight: 30,
                },
                EvaluationBuilder {
                    name: "Informe Final".to_string(),
                    weight: 40,
                },
                EvaluationBuilder {
                    name: "Evaluación Empresa".to_string(),
                    weight: 30,
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
            "teacherId": self.teacher_id
        })
    }
}

pub fn generate_unique_code() -> String {
    // Use UUID to ensure uniqueness, convert to numeric value
    let uuid = Uuid::new_v4();
    let uuid_bytes = uuid.as_bytes();
    let numeric_value = u32::from_be_bytes([
        uuid_bytes[0],
        uuid_bytes[1],
        uuid_bytes[2],
        uuid_bytes[3],
    ]);
    let four_digits = (numeric_value % 9000) + 1000; // Ensures 4 digits between 1000-9999
    format!("INFO{:04}", four_digits)
}

pub fn generate_unique_name() -> String {
    format!(
        "Test Asignature {}",
        Uuid::new_v4().to_string()[0..8].to_uppercase()
    )
}

pub async fn create_asignature(
    server: &axum_test::TestServer,
    asignature: &Value,
) -> Value {
    let response = server.post("/asignatures").json(&asignature).await;
    let body = response.json::<ResponseBody>();
    assert_eq!(response.status_code(), 201, "Failed to create asignature");
    body.data
}
