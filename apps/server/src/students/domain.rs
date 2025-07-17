use uuid::Uuid;

pub struct Student {
    pub id: Uuid,
    pub user_id: Uuid,
    pub asignature_id: Uuid,
    pub practice_id: Uuid,
    pub evaluation_scores: Vec<f32>,
    pub status: String,
}
