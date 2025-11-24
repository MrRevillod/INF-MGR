use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Configuration for plagiarism detection
#[derive(Debug, Clone)]
pub struct PlagiarismConfig {
    /// Minimum similarity score to consider a match (0.0 - 1.0)
    /// Higher values = stricter matching, only very similar content
    pub similarity_threshold: f32,
    
    /// Minimum content length in characters to analyze a chunk
    /// Shorter chunks are skipped as they don't provide meaningful comparison
    pub min_content_length: usize,
    
    /// Minimum score threshold for Qdrant search (pre-filter)
    /// This filters results before similarity_threshold is applied
    pub search_threshold: f32,
    
    /// Maximum number of similar chunks to retrieve from Qdrant per query
    pub search_limit: usize,
}

impl Default for PlagiarismConfig {
    fn default() -> Self {
        Self {
            similarity_threshold: 0.85,  // Only flag highly similar content (85%+)
            min_content_length: 150,     // Require substantial content
            search_threshold: 0.75,      // Pre-filter at 75% to reduce noise
            search_limit: 50,            // Retrieve top 50 matches per chunk
        }
    }
}

/// Represents a single plagiarism match between two chunks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlagiarismMatch {
    /// ID of the source chunk being analyzed
    pub source_chunk_id: Uuid,
    
    /// ID of the matched chunk found in another practice
    pub matched_chunk_id: Uuid,
    
    /// Practice ID where the match was found
    pub matched_practice_id: Uuid,
    
    /// Similarity score (0.0 - 1.0)
    pub similarity_score: f32,
    
    /// Content of the source chunk
    pub source_content: String,
    
    /// Content of the matched chunk
    pub matched_content: String,
    
    /// Section title of the source chunk
    pub source_section: String,
    
    /// Section title of the matched chunk
    pub matched_section: String,
    
    /// Root section of the source chunk (for verification)
    pub source_root_section: String,
    
    /// Root section of the matched chunk (should always equal source_root_section)
    pub matched_root_section: String,
}

/// Analysis result for a single practice document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlagiarismAnalysisResult {
    /// Practice ID that was analyzed
    pub practice_id: Uuid,
    
    /// Total number of chunks analyzed
    pub total_chunks_analyzed: usize,
    
    /// Total number of matches found across all practices
    pub total_matches_found: usize,
    
    /// All plagiarism matches found, grouped by matched practice
    pub matches: Vec<PlagiarismMatch>,
    
    /// Summary statistics per matched practice
    pub practice_summaries: Vec<PracticeSummary>,
    
    /// Whether plagiarism was detected (based on heuristics)
    pub has_significant_plagiarism: bool,
    
    /// Overall similarity score with the most similar practice (0.0 - 1.0)
    pub max_similarity_score: f32,
}

/// Summary of matches with a specific practice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PracticeSummary {
    /// Practice ID
    pub practice_id: Uuid,
    
    /// Number of matches with this practice
    pub match_count: usize,
    
    /// Average similarity score across all matches
    pub avg_similarity: f32,
    
    /// Maximum similarity score found
    pub max_similarity: f32,
    
    /// Number of high-quality matches (>0.95 similarity)
    pub high_quality_matches: usize,
    
    /// Percentage of source chunks that matched with this practice
    pub coverage_percentage: f32,
}
