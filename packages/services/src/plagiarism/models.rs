use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PlagiarismReport {
    pub id: Uuid,
    pub source_practice_id: Uuid,
    pub total_chunks_analyzed: i32,
    pub total_matches_found: i32,
    pub overall_similarity_score: f32,
    pub has_plagiarism: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlagiarismReportWithMatches {
    pub id: Uuid,
    pub source_practice_id: Uuid,
    pub total_chunks_analyzed: i32,
    pub total_matches_found: i32,
    pub overall_similarity_score: f32,
    pub has_plagiarism: bool,
    pub created_at: DateTime<Utc>,
    pub matches: Vec<PlagiarismMatch>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct PlagiarismMatch {
    pub id: Uuid,
    pub report_id: Uuid,
    pub source_chunk_id: Uuid,
    pub matched_chunk_id: Uuid,
    pub matched_practice_id: Uuid,
    pub similarity_score: f32,
    pub source_content: String,
    pub matched_content: String,
    pub source_section: String,
    pub matched_section: String,
}

#[derive(Debug, Clone)]
pub struct CreatePlagiarismReport {
    pub source_practice_id: Uuid,
    pub total_chunks_analyzed: i32,
    pub total_matches_found: i32,
    pub overall_similarity_score: f32,
    pub has_plagiarism: bool,
}

#[derive(Debug, Clone)]
pub struct CreatePlagiarismMatch {
    pub report_id: Uuid,
    pub source_chunk_id: Uuid,
    pub matched_chunk_id: Uuid,
    pub matched_practice_id: Uuid,
    pub similarity_score: f32,
    pub source_content: String,
    pub matched_content: String,
    pub source_section: String,
    pub matched_section: String,
}

// Configuration for plagiarism detection
#[derive(Debug, Clone)]
pub struct PlagiarismConfig {
    pub similarity_threshold: f32, // e.g., 0.85 = 85% similarity
    pub min_content_length: usize, // Minimum chars to consider for analysis
    pub plagiarism_threshold: f32, // Overall % of matches to flag as plagiarism
}

impl Default for PlagiarismConfig {
    fn default() -> Self {
        Self {
            similarity_threshold: 0.85, // Much higher - only flag very similar content
            min_content_length: 150,    // Longer chunks for better quality
            plagiarism_threshold: 0.40, // Require 40% of content to match (not just structure)
        }
    }
}

impl From<(PlagiarismReport, Vec<PlagiarismMatch>)> for PlagiarismReportWithMatches {
    fn from((report, matches): (PlagiarismReport, Vec<PlagiarismMatch>)) -> Self {
        Self {
            id: report.id,
            source_practice_id: report.source_practice_id,
            total_chunks_analyzed: report.total_chunks_analyzed,
            total_matches_found: report.total_matches_found,
            overall_similarity_score: report.overall_similarity_score,
            has_plagiarism: report.has_plagiarism,
            created_at: report.created_at,
            matches,
        }
    }
}
