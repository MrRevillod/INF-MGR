use std::collections::HashMap;
use tex_parser::ParsedTex;
use uuid::Uuid;

use crate::{
    embeddings::{EmbeddingChunk, EmbeddingService},
    plagiarism_new::{
        PlagiarismAnalysisResult, PlagiarismConfig, PlagiarismMatch, PracticeSummary,
    },
};

/// Service for detecting plagiarism in LaTeX practice reports
/// 
/// This service implements a clean, focused plagiarism detection algorithm:
/// 1. Parse LaTeX document into hierarchical chunks (handled by tex_parser)
/// 2. Filter chunks by minimum content length
/// 3. Vectorize each chunk using embeddings
/// 4. Search for similar chunks ONLY within the same root_section
/// 5. Calculate similarity metrics and detect plagiarism patterns
/// 
/// Key principles:
/// - Only compare "pears with pears" (same root_section)
/// - Template sections are excluded by the parser (Descripción empresa, Organigrama)
/// - Empty chunks are automatically filtered by min_content_length
/// - Focus on substantive content, not document structure
#[derive(Clone)]
pub struct PlagiarismDetectionService {
    embedding_service: EmbeddingService,
    config: PlagiarismConfig,
}

impl PlagiarismDetectionService {
    /// Create a new plagiarism detection service with default configuration
    pub fn new(embedding_service: EmbeddingService) -> Self {
        Self {
            embedding_service,
            config: PlagiarismConfig::default(),
        }
    }

    /// Create a new plagiarism detection service with custom configuration
    pub fn with_config(
        embedding_service: EmbeddingService,
        config: PlagiarismConfig,
    ) -> Self {
        Self {
            embedding_service,
            config,
        }
    }

    /// Analyze a parsed LaTeX document for plagiarism
    /// 
    /// Process:
    /// 1. Filter chunks by minimum content length (skip empty/short chunks)
    /// 2. Save each chunk to Qdrant with its embedding
    /// 3. Search for similar chunks within the SAME root_section
    /// 4. Collect matches that exceed similarity_threshold
    /// 5. Calculate statistics and detect plagiarism patterns
    /// 
    /// Returns a detailed analysis result with all matches and statistics
    pub async fn analyze_document(
        &self,
        practice_id: Uuid,
        parsed_tex: ParsedTex,
    ) -> Result<PlagiarismAnalysisResult, Box<dyn std::error::Error>> {
        println!("\n🔍 Iniciando análisis de plagio para práctica {}", practice_id);
        println!("📄 Total de chunks parseados: {}", parsed_tex.chunks.len());

        // Step 1: Filter chunks by minimum content length
        // This automatically removes:
        // - Empty parent sections (e.g., "Actividades encomendadas" with no direct text)
        // - Very short sections that don't provide meaningful comparison
        // - Sections excluded by parser (Descripción empresa, Organigrama)
        let chunks_to_analyze: Vec<EmbeddingChunk> = parsed_tex
            .chunks
            .iter()
            .filter(|chunk| {
                let content_length = chunk.content.trim().len();
                
                if content_length < self.config.min_content_length {
                    println!(
                        "   ⏭️  Skipping chunk '{}' (root: {}): too short ({} chars < {} min)",
                        chunk.title,
                        chunk.root_section,
                        content_length,
                        self.config.min_content_length
                    );
                    return false;
                }

                true
            })
            .map(|chunk| EmbeddingChunk::from((&practice_id, chunk.clone())))
            .collect();

        let total_chunks = chunks_to_analyze.len();
        println!("✅ Chunks a analizar después de filtrado: {}", total_chunks);

        if total_chunks == 0 {
            println!("⚠️  No hay chunks para analizar");
            return Ok(PlagiarismAnalysisResult {
                practice_id,
                total_chunks_analyzed: 0,
                total_matches_found: 0,
                matches: vec![],
                practice_summaries: vec![],
                has_significant_plagiarism: false,
                max_similarity_score: 0.0,
            });
        }

        // Step 2 & 3: Save chunks and search for similar ones
        // We use a HashMap to track the BEST match per (source_chunk, target_practice) pair
        // This prevents duplicate matches and keeps only the highest similarity score
        let mut best_matches: HashMap<(Uuid, Uuid), PlagiarismMatch> = HashMap::new();

        for (idx, chunk) in chunks_to_analyze.iter().enumerate() {
            println!(
                "\n   [{}/{}] Analizando chunk '{}' (root: {}, {} chars)",
                idx + 1,
                total_chunks,
                chunk.title,
                chunk.root_section,
                chunk.content.len()
            );

            // Save the chunk to Qdrant (with its embedding)
            self.embedding_service
                .save_chunk(chunk.clone())
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            // Search for similar chunks in the SAME root_section
            // This is the critical "pears with pears" comparison
            let similar_chunks = self
                .embedding_service
                .find_similar_chunks(
                    &chunk.content,
                    &chunk.root_section,  // Only match same root_section
                    practice_id,          // Exclude own practice
                    self.config.search_limit,
                    self.config.search_threshold,
                )
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            println!(
                "      Qdrant retornó {} chunks similares (mismo root_section: '{}')",
                similar_chunks.similar_chunks.len(),
                chunk.root_section
            );

            // Step 4: Filter and collect matches above similarity_threshold
            let high_quality_matches: Vec<_> = similar_chunks
                .similar_chunks
                .into_iter()
                .filter(|m| m.similarity_score >= self.config.similarity_threshold)
                .collect();

            if !high_quality_matches.is_empty() {
                println!(
                    "      ✓ {} matches de alta calidad (>= {:.0}%)",
                    high_quality_matches.len(),
                    self.config.similarity_threshold * 100.0
                );
            }

            // Process each match and keep only the best per target practice
            for similar in high_quality_matches {
                let key = (chunk.id, similar.chunk.practice_id);

                // Create the match record
                let plagiarism_match = PlagiarismMatch {
                    source_chunk_id: chunk.id,
                    matched_chunk_id: similar.chunk.id,
                    matched_practice_id: similar.chunk.practice_id,
                    similarity_score: similar.similarity_score,
                    source_content: chunk.content.clone(),
                    matched_content: similar.chunk.content.clone(),
                    source_section: chunk.title.clone(),
                    matched_section: similar.chunk.title.clone(),
                    source_root_section: chunk.root_section.clone(),
                    matched_root_section: similar.chunk.root_section.clone(),
                };

                // Keep only the best match for this (chunk, practice) combination
                best_matches
                    .entry(key)
                    .and_modify(|existing| {
                        if plagiarism_match.similarity_score > existing.similarity_score {
                            *existing = plagiarism_match.clone();
                        }
                    })
                    .or_insert(plagiarism_match);
            }
        }

        // Step 5: Calculate statistics and create result
        let all_matches: Vec<PlagiarismMatch> = best_matches.into_values().collect();
        let total_matches = all_matches.len();

        println!("\n📊 Análisis completado:");
        println!("   Total de matches únicos encontrados: {}", total_matches);

        // Group matches by practice and calculate statistics
        let mut practice_stats: HashMap<Uuid, Vec<f32>> = HashMap::new();
        
        for m in &all_matches {
            practice_stats
                .entry(m.matched_practice_id)
                .or_insert_with(Vec::new)
                .push(m.similarity_score);
        }

        // Generate practice summaries
        let mut practice_summaries: Vec<PracticeSummary> = practice_stats
            .into_iter()
            .map(|(practice_id, scores)| {
                let match_count = scores.len();
                let avg_similarity = scores.iter().sum::<f32>() / match_count as f32;
                let max_similarity = scores.iter().cloned().fold(0.0f32, f32::max);
                let high_quality_matches = scores.iter().filter(|&&s| s >= 0.95).count();
                let coverage_percentage = (match_count as f32 / total_chunks as f32) * 100.0;

                PracticeSummary {
                    practice_id,
                    match_count,
                    avg_similarity,
                    max_similarity,
                    high_quality_matches,
                    coverage_percentage,
                }
            })
            .collect();

        // Sort by max_similarity descending to show most similar first
        practice_summaries.sort_by(|a, b| {
            b.max_similarity.partial_cmp(&a.max_similarity).unwrap()
        });

        // Print detailed statistics
        if !practice_summaries.is_empty() {
            println!("\n   Resumen por práctica:");
            for summary in &practice_summaries {
                println!(
                    "   • Práctica {}: {} matches, avg={:.1}%, max={:.1}%, high-quality={}, cobertura={:.1}%",
                    summary.practice_id,
                    summary.match_count,
                    summary.avg_similarity * 100.0,
                    summary.max_similarity * 100.0,
                    summary.high_quality_matches,
                    summary.coverage_percentage
                );
            }
        }

        // Detect significant plagiarism
        // Criteria:
        // 1. High coverage (>80%) with high-quality matches (>90% are 0.95+ similar)
        //    → Likely identical/copied document
        // 2. High max similarity (>0.92) with decent coverage (>50%)
        //    → Substantial copying
        let (has_significant_plagiarism, max_similarity_score) = 
            if let Some(top_summary) = practice_summaries.first() {
                let high_quality_ratio = 
                    top_summary.high_quality_matches as f32 / top_summary.match_count as f32;
                
                let is_identical = top_summary.coverage_percentage > 80.0 
                    && high_quality_ratio > 0.9;
                
                let is_substantial = top_summary.max_similarity > 0.92 
                    && top_summary.coverage_percentage > 50.0;
                
                let has_plagiarism = is_identical || is_substantial;
                
                if has_plagiarism {
                    println!("\n⚠️  PLAGIO DETECTADO:");
                    if is_identical {
                        println!("   Documento muy similar o idéntico detectado");
                    } else if is_substantial {
                        println!("   Coincidencias sustanciales detectadas");
                    }
                }
                
                (has_plagiarism, top_summary.max_similarity)
            } else {
                (false, 0.0)
            };

        Ok(PlagiarismAnalysisResult {
            practice_id,
            total_chunks_analyzed: total_chunks,
            total_matches_found: total_matches,
            matches: all_matches,
            practice_summaries,
            has_significant_plagiarism,
            max_similarity_score,
        })
    }
}
