use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

use crate::asignatures::domain::{Asignature, Evaluation};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct AsignatureModel {
    pub id: Uuid,
    pub year: i32,
    pub code: String,
    pub name: String,
    pub evaluations: Vec<EvaluationType>,

    #[sqlx(rename = "teacher_id")]
    pub teacher_id: Uuid,
}

impl From<AsignatureModel> for Asignature {
    fn from(asignature: AsignatureModel) -> Self {
        Asignature {
            id: asignature.id,
            year: asignature.year,
            code: asignature.code,
            name: asignature.name,
            evaluations: asignature
                .evaluations
                .into_iter()
                .map(Evaluation::from)
                .collect(),
            teacher_id: asignature.teacher_id,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "evaluation")]
pub struct EvaluationType {
    pub id: Uuid,
    pub name: String,
    pub weight: i32,
}

impl From<EvaluationType> for Evaluation {
    fn from(evaluation: EvaluationType) -> Self {
        Evaluation {
            id: evaluation.id,
            name: evaluation.name,
            weight: evaluation.weight,
        }
    }
}

impl From<Asignature> for AsignatureModel {
    fn from(asignature: Asignature) -> Self {
        AsignatureModel {
            id: asignature.id,
            year: asignature.year,
            code: asignature.code,
            name: asignature.name,
            evaluations: asignature
                .evaluations
                .into_iter()
                .map(EvaluationType::from)
                .collect(),
            teacher_id: asignature.teacher_id,
        }
    }
}

impl From<Evaluation> for EvaluationType {
    fn from(evaluation: Evaluation) -> Self {
        EvaluationType {
            id: evaluation.id,
            name: evaluation.name,
            weight: evaluation.weight,
        }
    }
}
