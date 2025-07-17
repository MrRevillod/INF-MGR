use uuid::Uuid;

pub struct Evaluation {
    pub name: String,
    pub score: f32,
    pub weight: f32,
}

pub struct Asignature {
    pub id: Uuid,
    pub year: u16,
    pub code: String,
    pub name: String,
    pub evaluations: Vec<Evaluation>,
    pub teacher_id: Uuid,
}
