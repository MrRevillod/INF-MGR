use std::collections::HashMap;
use tex_parser::ParsedTex;
use uuid::Uuid;

use crate::{
    embeddings::{EmbeddingChunk, EmbeddingService},
    plagiarism::{
        CreatePlagiarismMatch, CreatePlagiarismReport, PlagiarismConfig,
        PlagiarismReportWithMatches, PlagiarismRepository,
    },
};

#[derive(Clone)]
pub struct PlagiarismDetectionService {
    embedding_service: EmbeddingService,
    repository: PlagiarismRepository,
    config: PlagiarismConfig,
}

impl PlagiarismDetectionService {
    pub fn new(
        embedding_service: EmbeddingService,
        repository: PlagiarismRepository,
    ) -> Self {
        Self {
            embedding_service,
            repository,
            config: PlagiarismConfig::default(),
        }
    }

    pub fn with_config(
        embedding_service: EmbeddingService,
        repository: PlagiarismRepository,
        config: PlagiarismConfig,
    ) -> Self {
        Self {
            embedding_service,
            repository,
            config,
        }
    }

    pub async fn analyze_document(
        &self,
        practice_id: Uuid,
        parsed_tex: ParsedTex,
    ) -> Result<PlagiarismReportWithMatches, Box<dyn std::error::Error>> {
        // Skip sections that are standard templates or administrative
        let skipped_sections = vec![
            "Resumen",
            "Introducción",
            "Objetivos y expectativas",
            "Descripción de la empresa",
            "Organigrama de la empresa",
            "Actividades encomendadas",
            "Tecnologías aplicadas",
            "Experiencia en el proceso de práctica",
            "Misión",
            "Visión",
            "Funciones",
        ];

        // Generic phrases that appear in all academic reports (filter by content)
        let generic_phrases = vec![
            "Un stack tecnológico es el conjunto",
            "Se describen las principales actividades",
            "Se reflexiona sobre los aprendizajes",
            "Se explica el problema a resolver",
            "La experiencia resultó",
            "Se destaca el",
        ];

        dbg!(&parsed_tex);

        // Filter and convert chunks
        let chunks_to_analyze = parsed_tex
            .chunks
            .iter()
            .filter(|chunk| {
                // Skip if section title is in skip list
                if skipped_sections.contains(&chunk.title.as_str()) {
                    return false;
                }

                // Skip if content is too short
                if chunk.content.len() < self.config.min_content_length {
                    return false;
                }

                // Skip if content contains generic template phrases
                let content_lower = chunk.content.to_lowercase();
                if generic_phrases
                    .iter()
                    .any(|phrase| content_lower.contains(&phrase.to_lowercase()))
                {
                    return false;
                }

                true
            })
            .map(|chunk| EmbeddingChunk::from((&practice_id, chunk.clone())))
            .collect::<Vec<_>>();

        let total_chunks = chunks_to_analyze.len();

        // Track best match per (source_chunk, target_practice) to avoid duplicates
        let mut best_matches: HashMap<(Uuid, Uuid), CreatePlagiarismMatch> =
            HashMap::new();

        // Process each chunk
        for chunk in chunks_to_analyze {
            // Save the chunk first (if not already saved)
            self.embedding_service
                .save_chunk(chunk.clone())
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            // Search for similar chunks with strict threshold
            let similar_chunks = self
                .embedding_service
                .find_similar_chunks(
                    &chunk.content,
                    practice_id,
                    50,   // Reduced - only need top matches
                    0.80, // Higher threshold - only very similar content
                )
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

            println!(
                "   Chunk '{}': Qdrant retornó {} resultados similares",
                chunk.id,
                similar_chunks.similar_chunks.len()
            );

            // Process matches - keep only the best match per target practice
            for similar in similar_chunks.similar_chunks {
                if similar.similarity_score >= self.config.similarity_threshold {
                    // Validate that the matched practice exists in enrollments
                    let practice_exists = self
                        .repository
                        .practice_exists(similar.chunk.practice_id)
                        .await
                        .unwrap_or(false); // If error occurs, assume it doesn't exist

                    if !practice_exists {
                        println!(
                            "⚠️  Skipping match with non-existent practice: {}",
                            similar.chunk.practice_id
                        );
                        continue; // Skip this match
                    }

                    // Normalize similarity score to ensure it's within [0.0, 1.0]
                    let normalized_score = similar.similarity_score.min(1.0).max(0.0);

                    let key = (chunk.id, similar.chunk.practice_id);

                    // Check if we already have a match for this (source_chunk, target_practice) pair
                    let should_update = match best_matches.get(&key) {
                        Some(existing) => normalized_score > existing.similarity_score,
                        None => true,
                    };

                    if should_update {
                        println!(
                            "      ✓ Match válido: score={:.4}, sección='{}' vs '{}' (práctica: {})",
                            normalized_score,
                            chunk.root_section,
                            similar.chunk.root_section,
                            similar.chunk.practice_id
                        );

                        let match_data = CreatePlagiarismMatch {
                            report_id: Uuid::new_v4(), // Will be updated after report creation
                            source_chunk_id: chunk.id,
                            matched_chunk_id: similar.chunk.id,
                            matched_practice_id: similar.chunk.practice_id,
                            similarity_score: normalized_score,
                            source_content: chunk.content.clone(),
                            matched_content: similar.chunk.content.clone(),
                            source_section: chunk.root_section.clone(),
                            matched_section: similar.chunk.root_section,
                        };

                        best_matches.insert(key, match_data);
                    }
                }
            }
        }

        // Convert best_matches HashMap to Vec for processing
        let all_matches: Vec<CreatePlagiarismMatch> =
            best_matches.into_values().collect();

        // Group matches by target practice to calculate per-practice similarity
        let mut practice_similarities: HashMap<Uuid, Vec<f32>> = HashMap::new();

        for m in &all_matches {
            practice_similarities
                .entry(m.matched_practice_id)
                .or_insert_with(Vec::new)
                .push(m.similarity_score);
        }

        // Find the practice with highest average similarity
        let (overall_similarity, _dominant_practice) = if !practice_similarities
            .is_empty()
        {
            let mut best_avg = 0.0f32;
            let mut best_practice = None;

            for (practice_id, scores) in &practice_similarities {
                let avg: f32 = scores.iter().sum::<f32>() / scores.len() as f32;
                let max_score = scores.iter().cloned().fold(0.0f32, f32::max);
                let high_quality_matches = scores.iter().filter(|&&s| s > 0.95).count();

                // For identical documents: if >90% of analyzed chunks have high-quality matches
                let coverage = scores.len() as f32 / total_chunks as f32;
                let identical_doc_score =
                    if high_quality_matches as f32 / scores.len() as f32 > 0.9
                        && coverage > 0.8
                    {
                        max_score // Use max score for identical documents
                    } else {
                        avg // Use average for partial plagiarism
                    };

                if identical_doc_score > best_avg {
                    best_avg = identical_doc_score;
                    best_practice = Some(*practice_id);
                }
            }

            println!("\n📊 Análisis de similitud por práctica:");
            for (practice_id, scores) in &practice_similarities {
                let avg: f32 = scores.iter().sum::<f32>() / scores.len() as f32;
                let max_score = scores.iter().cloned().fold(0.0f32, f32::max);
                let high_quality = scores.iter().filter(|&&s| s > 0.95).count();
                println!(
                    "   Práctica {}: {} matches, avg={:.2}%, max={:.2}%, high-quality={}",
                    practice_id,
                    scores.len(),
                    avg * 100.0,
                    max_score * 100.0,
                    high_quality
                );
            }

            (best_avg, best_practice)
        } else {
            (0.0, None)
        };

        // Calculate overall statistics
        let match_count = all_matches.len();
        let match_percentage = if total_chunks > 0 {
            match_count as f32 / total_chunks as f32
        } else {
            0.0
        };

        println!(
            "\n📈 Métricas globales: similarity={:.2}%, matches={}/{}, coverage={:.2}%",
            overall_similarity * 100.0,
            match_count,
            total_chunks,
            match_percentage * 100.0
        );

        // Recalculate with simplified logic (already computed above)
        // Note: overall_similarity was already calculated considering identical docs
        let match_count = all_matches.len();

        let match_percentage = if total_chunks > 0 {
            match_count as f32 / total_chunks as f32
        } else {
            0.0
        };

        // Simple plagiarism detection: high similarity + significant match percentage
        let has_plagiarism = overall_similarity >= self.config.similarity_threshold
            && match_percentage >= self.config.plagiarism_threshold;

        // Create report
        let report_data = CreatePlagiarismReport {
            source_practice_id: practice_id,
            total_chunks_analyzed: total_chunks as i32,
            total_matches_found: all_matches.len() as i32,
            overall_similarity_score: overall_similarity,
            has_plagiarism,
        };

        let report = self
            .repository
            .create_report(report_data)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        // Update matches with correct report_id
        if !all_matches.is_empty() {
            let updated_matches = all_matches
                .into_iter()
                .map(|mut m| {
                    m.report_id = report.id;
                    m
                })
                .collect();

            self.repository
                .create_matches_batch(updated_matches)
                .await
                .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        }

        // Load the complete report with matches
        let complete_report = self
            .repository
            .get_report_by_practice_id(practice_id)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?
            .ok_or("Failed to load created report")?;

        Ok(complete_report)
    }

    pub async fn get_report(
        &self,
        practice_id: Uuid,
    ) -> Result<Option<PlagiarismReportWithMatches>, Box<dyn std::error::Error>> {
        let report = self
            .repository
            .get_report_by_practice_id(practice_id)
            .await
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

        Ok(report)
    }
}
