use uuid::Uuid;

use crate::asignatures::domain::{Asignature, Evaluation};

#[derive(Debug, Clone)]
pub struct CreateAsignatureInput {
    pub year: i16,
    pub code: String,
    pub name: String,
    pub evaluations: Vec<Evaluation>,
    pub teacher_id: Uuid,
}

impl From<CreateAsignatureInput> for Asignature {
    fn from(input: CreateAsignatureInput) -> Self {
        Asignature {
            id: Uuid::new_v4(),
            year: input.year,
            code: input.code,
            name: input.name,
            evaluations: input.evaluations,
            teacher_id: input.teacher_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UpdateAsignatureInput {
    pub year: Option<i16>,
    pub code: Option<String>,
    pub name: Option<String>,
    pub evaluations: Option<Vec<Evaluation>>,
    pub teacher_id: Option<Uuid>,
}
