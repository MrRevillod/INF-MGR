use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

use crate::{
    asignatures::{domain::Asignature, infrastructure::AsignatureModel},
    inscriptions::domain::{Inscription, InscriptionError, StudentEvaluation},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "student_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum StudentStatus {
    Active,
    Inactive,
    Completed,
    Evaluating,
}

impl FromStr for StudentStatus {
    type Err = InscriptionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(StudentStatus::Active),
            "inactive" => Ok(StudentStatus::Inactive),
            "completed" => Ok(StudentStatus::Completed),
            "evaluating" => Ok(StudentStatus::Evaluating),
            _ => Err(InscriptionError::InvalidStatus {
                status: s.to_string(),
            }),
        }
    }
}

impl Display for StudentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match self {
            StudentStatus::Active => "active",
            StudentStatus::Inactive => "inactive",
            StudentStatus::Completed => "completed",
            StudentStatus::Evaluating => "evaluating",
        };

        write!(f, "{status}")
    }
}

impl From<StudentStatus> for String {
    fn from(status: StudentStatus) -> Self {
        status.to_string()
    }
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct InscriptionModel {
    pub id: Uuid,

    #[sqlx(rename = "user_id")]
    pub user_id: Uuid,

    #[sqlx(rename = "asignature_id")]
    pub asignature_id: Uuid,

    #[sqlx(rename = "practice_id")]
    pub practice_id: Option<Uuid>,

    #[sqlx(rename = "evaluations_scores")]
    pub evaluations_scores: Vec<StudentEvaluationModel>,
    pub status: StudentStatus,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct InscriptionResponseModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub asignature_id: Uuid,
    pub practice_id: Option<Uuid>,
    pub evaluations_scores: Vec<StudentEvaluationModel>,
    pub status: StudentStatus,
    pub asignature: AsignatureModel,
}

impl From<(Asignature, Inscription)> for InscriptionResponseModel {
    fn from((asignature, inscription): (Asignature, Inscription)) -> Self {
        InscriptionResponseModel {
            id: inscription.id,
            user_id: inscription.user_id,
            asignature_id: inscription.asignature_id,
            practice_id: inscription.practice_id,
            evaluations_scores: inscription
                .evaluations_scores
                .into_iter()
                .map(Into::into)
                .collect(),

            status: StudentStatus::from_str(&inscription.status)
                .unwrap_or(StudentStatus::Active),

            asignature: AsignatureModel::from(asignature),
        }
    }
}

impl From<InscriptionModel> for Inscription {
    fn from(value: InscriptionModel) -> Self {
        Inscription {
            id: value.id,
            user_id: value.user_id,
            asignature_id: value.asignature_id,
            practice_id: value.practice_id,
            evaluations_scores: value
                .evaluations_scores
                .into_iter()
                .map(Into::into)
                .collect(),

            status: value.status.to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "student_evaluation")]
pub struct StudentEvaluationModel {
    pub id: Uuid,
    pub score: f64,
}

impl From<StudentEvaluationModel> for StudentEvaluation {
    fn from(value: StudentEvaluationModel) -> Self {
        StudentEvaluation {
            id: value.id,
            score: value.score,
        }
    }
}

impl From<Inscription> for InscriptionModel {
    fn from(value: Inscription) -> Self {
        InscriptionModel {
            id: value.id,
            user_id: value.user_id,
            asignature_id: value.asignature_id,
            practice_id: value.practice_id,
            evaluations_scores: value
                .evaluations_scores
                .into_iter()
                .map(Into::into)
                .collect(),

            status: StudentStatus::from_str(&value.status)
                .unwrap_or(StudentStatus::Active),
        }
    }
}

impl From<StudentEvaluation> for StudentEvaluationModel {
    fn from(value: StudentEvaluation) -> Self {
        StudentEvaluationModel {
            id: value.id,
            score: value.score,
        }
    }
}
