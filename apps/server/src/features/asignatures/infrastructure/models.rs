use std::str::FromStr;

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

use crate::asignatures::domain::{Asignature, AsignatureError, Evaluation};

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
    pub status: AsignatureStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[sqlx(type_name = "asignature_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AsignatureStatus {
    InProgress,
    Ended,
}

impl FromStr for AsignatureStatus {
    type Err = AsignatureError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inprogress" => Ok(AsignatureStatus::InProgress),
            "ended" => Ok(AsignatureStatus::Ended),
            _ => Err(AsignatureError::UknownError(
                "Unknown asignature status".to_string(),
            )),
        }
    }
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
            status: match asignature.status {
                AsignatureStatus::InProgress => "inprogress".to_string(),
                AsignatureStatus::Ended => "ended".to_string(),
            },
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
            status: match asignature.status.as_str() {
                "inprogress" => AsignatureStatus::InProgress,
                "ended" => AsignatureStatus::Ended,
                _ => panic!("Unknown asignature status"),
            },
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
