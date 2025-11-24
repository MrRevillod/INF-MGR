use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::plagiarism::{
    CreatePlagiarismMatch, CreatePlagiarismReport, PlagiarismMatch, PlagiarismReport,
    PlagiarismReportWithMatches,
};

#[derive(Clone)]
pub struct PlagiarismRepository {
    pool: Pool<Postgres>,
}

impl PlagiarismRepository {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    /// Check if a practice_id exists in enrollments (has an active enrollment)
    pub async fn practice_exists(&self, practice_id: Uuid) -> Result<bool, sqlx::Error> {
        let count: i64 =
            sqlx::query_scalar("SELECT COUNT(*) FROM enrollments WHERE practice_id = $1")
                .bind(practice_id)
                .fetch_one(&self.pool)
                .await?;

        Ok(count > 0)
    }

    pub async fn create_report(
        &self,
        report: CreatePlagiarismReport,
    ) -> Result<PlagiarismReport, sqlx::Error> {
        let record = sqlx::query_as::<_, PlagiarismReport>(
            r#"
            INSERT INTO plagiarism_reports (
                source_practice_id, 
                total_chunks_analyzed, 
                total_matches_found, 
                overall_similarity_score, 
                has_plagiarism
            )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, source_practice_id, total_chunks_analyzed, 
                     total_matches_found, overall_similarity_score, 
                     has_plagiarism, created_at
            "#,
        )
        .bind(report.source_practice_id)
        .bind(report.total_chunks_analyzed)
        .bind(report.total_matches_found)
        .bind(report.overall_similarity_score)
        .bind(report.has_plagiarism)
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }

    pub async fn create_match(
        &self,
        match_data: CreatePlagiarismMatch,
    ) -> Result<PlagiarismMatch, sqlx::Error> {
        let record = sqlx::query_as::<_, PlagiarismMatch>(
            r#"
            INSERT INTO plagiarism_matches (
                report_id, source_chunk_id, matched_chunk_id, 
                matched_practice_id, similarity_score, source_content, 
                matched_content, source_section, matched_section
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id, report_id, source_chunk_id, matched_chunk_id, 
                     matched_practice_id, similarity_score, source_content, 
                     matched_content, source_section, matched_section
            "#,
        )
        .bind(match_data.report_id)
        .bind(match_data.source_chunk_id)
        .bind(match_data.matched_chunk_id)
        .bind(match_data.matched_practice_id)
        .bind(match_data.similarity_score)
        .bind(match_data.source_content)
        .bind(match_data.matched_content)
        .bind(match_data.source_section)
        .bind(match_data.matched_section)
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }

    pub async fn get_report_by_practice_id(
        &self,
        practice_id: Uuid,
    ) -> Result<Option<PlagiarismReportWithMatches>, sqlx::Error> {
        let report = sqlx::query_as::<_, PlagiarismReport>(
            r#"
            SELECT id, source_practice_id, total_chunks_analyzed, 
                   total_matches_found, overall_similarity_score, 
                   has_plagiarism, created_at
            FROM plagiarism_reports
            WHERE source_practice_id = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(practice_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(report) = report {
            // Load matches for this report
            let matches = sqlx::query_as::<_, PlagiarismMatch>(
                r#"
                SELECT id, report_id, source_chunk_id, matched_chunk_id, 
                       matched_practice_id, similarity_score, source_content, 
                       matched_content, source_section, matched_section
                FROM plagiarism_matches
                WHERE report_id = $1
                ORDER BY similarity_score DESC
                "#,
            )
            .bind(report.id)
            .fetch_all(&self.pool)
            .await?;

            Ok(Some(PlagiarismReportWithMatches::from((report, matches))))
        } else {
            Ok(None)
        }
    }

    pub async fn create_matches_batch(
        &self,
        matches: Vec<CreatePlagiarismMatch>,
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.pool.begin().await?;

        for match_data in matches {
            sqlx::query(
                r#"
                INSERT INTO plagiarism_matches (
                    report_id, source_chunk_id, matched_chunk_id, 
                    matched_practice_id, similarity_score, source_content, 
                    matched_content, source_section, matched_section
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
                "#,
            )
            .bind(match_data.report_id)
            .bind(match_data.source_chunk_id)
            .bind(match_data.matched_chunk_id)
            .bind(match_data.matched_practice_id)
            .bind(match_data.similarity_score)
            .bind(match_data.source_content)
            .bind(match_data.matched_content)
            .bind(match_data.source_section)
            .bind(match_data.matched_section)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }
}
