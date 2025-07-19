use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

use crate::inscriptions::domain::{
    Inscription, InscriptionError, StudentEvaluation,
};

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

    #[sqlx(rename = "evaluation_scores")]
    pub evaluation_scores: Vec<StudentEvaluationModel>,
    pub status: String,
}

impl From<InscriptionModel> for Inscription {
    fn from(value: InscriptionModel) -> Self {
        Inscription {
            id: value.id,
            user_id: value.user_id,
            asignature_id: value.asignature_id,
            practice_id: value.practice_id,
            evaluation_scores: value
                .evaluation_scores
                .into_iter()
                .map(Into::into)
                .collect(),
            status: value.status,
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

#[derive(Serialize, Deserialize, Debug, Clone, Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "user_role", rename_all = "lowercase")]
pub enum InscriptionStatus {
    Active,
    Inactive,
    Completed,
    Evaluating,
}

impl Display for InscriptionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status_str = match self {
            InscriptionStatus::Active => "active",
            InscriptionStatus::Inactive => "inactive",
            InscriptionStatus::Completed => "completed",
            InscriptionStatus::Evaluating => "evaluating",
        };

        write!(f, "{status_str}")
    }
}

impl FromStr for InscriptionStatus {
    type Err = InscriptionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(InscriptionStatus::Active),
            "inactive" => Ok(InscriptionStatus::Inactive),
            "completed" => Ok(InscriptionStatus::Completed),
            "evaluating" => Ok(InscriptionStatus::Evaluating),
            _ => Err(InscriptionError::InvalidStudentState),
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
            evaluation_scores: value
                .evaluation_scores
                .into_iter()
                .map(Into::into)
                .collect(),
            status: value.status,
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
