#![cfg(feature = "seeder")]

use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::{collections::HashSet, vec};
use uuid::Uuid;

use server::asignatures::infrastructure::EvaluationType;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Asignature {
    pub id: Uuid,
    pub year: i32,
    pub code: String,
    pub name: String,
    pub teacher_id: Uuid,
    pub coordinator_id: Uuid,
}

pub async fn seed_asignatures(pool: &PgPool) -> Result<Vec<Uuid>, sqlx::Error> {
    // Obtener todos los teachers
    let teachers: Vec<Uuid> =
        sqlx::query_scalar("SELECT id FROM users WHERE $1::user_role = ANY(roles)")
            .bind("teacher")
            .fetch_all(pool)
            .await?;

    // Obtener todos los coordinators
    let coordinators: Vec<Uuid> =
        sqlx::query_scalar("SELECT id FROM users WHERE $1::user_role = ANY(roles)")
            .bind("coordinator")
            .fetch_all(pool)
            .await?;

    let mut asignature_ids = Vec::new();

    for i in 0..10 {
        // Elegir teacher aleatorio
        let teacher_idx = rand::random_range(0..teachers.len());
        let teacher_id = teachers[teacher_idx];

        // Elegir coordinator aleatorio
        let coordinator_idx = rand::random_range(0..coordinators.len());
        let coordinator_id = coordinators[coordinator_idx];

        // Generar datos de la asignatura
        let id = Uuid::new_v4();
        let code = format!("INFO{:03}", i + 1);
        let name = format!("Asignatura {}", i + 1);
        let year = 2025;
        let status = "inprogress";

        let evaluations = vec![
            EvaluationType {
                id: Uuid::new_v4(),
                name: String::from("Informe de Práctica"),
                weight: 30,
            },
            EvaluationType {
                id: Uuid::new_v4(),
                name: String::from("Bitácoras de Práctica"),
                weight: 30,
            },
            EvaluationType {
                id: Uuid::new_v4(),
                name: String::from("Evaluación del Supervisor de Práctica"),
                weight: 40,
            },
        ];

        // Insertar la asignatura, evaluations como ARRAY[]::evaluation[] y status casteado a asignature_status
        sqlx::query(
            r#"INSERT INTO asignatures (id, year, code, name, evaluations, teacher_id, coordinator_id, status)
            VALUES ($1, $2, $3, $4, $5::evaluation[], $6, $7, $8::asignature_status)"#
        )
        .bind(id)
        .bind(year)
        .bind(&code)
        .bind(&name)
        .bind(&evaluations)
        .bind(teacher_id)
        .bind(coordinator_id)
        .bind(status)
        .execute(pool)
        .await?;

        asignature_ids.push(id);
    }

    Ok(asignature_ids)
}

pub async fn seed_inscriptions(
    pool: &PgPool,
    asignature_ids: &[Uuid],
) -> Result<(), sqlx::Error> {
    // Obtener students
    let students: Vec<Uuid> =
        sqlx::query_scalar("SELECT id FROM users WHERE $1::user_role = ANY(roles)")
            .bind("student")
            .fetch_all(pool)
            .await?;
    let mut rng = rand::rng();
    let mut already_enrolled = HashSet::new();

    // 1. Garantizar que cada estudiante tenga al menos una inscripción
    for student_id in &students {
        let asignature_idx = rand::random_range(0..asignature_ids.len());
        let asignature_id = asignature_ids[asignature_idx];
        if already_enrolled.insert((*student_id, asignature_id)) {
            let id = Uuid::new_v4();
            sqlx::query(
                r#"INSERT INTO inscriptions (id, user_id, asignature_id, evaluations_scores, status)
                VALUES ($1, $2, $3, ARRAY[]::student_evaluation[], $4::student_status)
                ON CONFLICT (user_id, asignature_id) DO NOTHING"#
            )
            .bind(id)
            .bind(student_id)
            .bind(asignature_id)
            .bind("active")
            .execute(pool)
            .await?;
        }
    }

    // 2. Inscribir aleatoriamente estudiantes adicionales en asignaturas
    for asignature_id in asignature_ids {
        // Inscribir entre 10 y 20 estudiantes por asignatura
        let n = rand::random_range(10..=20);
        let mut selected = students.clone();
        selected.shuffle(&mut rng);
        for student_id in selected.iter().take(n) {
            if already_enrolled.insert((*student_id, *asignature_id)) {
                let id = Uuid::new_v4();
                sqlx::query(
                    r#"INSERT INTO inscriptions (id, user_id, asignature_id, evaluations_scores, status)
                    VALUES ($1, $2, $3, ARRAY[]::student_evaluation[], $4::student_status)
                    ON CONFLICT (user_id, asignature_id) DO NOTHING"#
                )
                .bind(id)
                .bind(student_id)
                .bind(asignature_id)
                .bind("active")
                .execute(pool)
                .await?;
            }
        }
    }
    Ok(())
}
