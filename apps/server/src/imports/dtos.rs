use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportedStudent {
    pub id: String,
    pub rut: String,
    pub name: String,
    pub email: String,
    pub register: String,
}
