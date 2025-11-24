CREATE TABLE IF NOT EXISTS plagiarism_reports (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    source_practice_id UUID NOT NULL REFERENCES practices(id) ON DELETE CASCADE,
    total_chunks_analyzed INTEGER NOT NULL,
    total_matches_found INTEGER NOT NULL,
    overall_similarity_score REAL NOT NULL,
    has_plagiarism BOOLEAN NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT valid_similarity_score CHECK (overall_similarity_score >= 0.0 AND overall_similarity_score <= 1.0)
);

CREATE TABLE IF NOT EXISTS plagiarism_matches (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    report_id UUID NOT NULL REFERENCES plagiarism_reports(id) ON DELETE CASCADE,
    source_chunk_id UUID NOT NULL,
    matched_chunk_id UUID NOT NULL,
    matched_practice_id UUID NOT NULL REFERENCES practices(id) ON DELETE CASCADE,
    similarity_score REAL NOT NULL,
    source_content TEXT NOT NULL,
    matched_content TEXT NOT NULL,
    source_section VARCHAR(255) NOT NULL,
    matched_section VARCHAR(255) NOT NULL,
    
    CONSTRAINT valid_match_similarity_score CHECK (similarity_score >= 0.0 AND similarity_score <= 1.0)
);

CREATE INDEX IF NOT EXISTS plagiarism_reports_source_practice_id_idx ON plagiarism_reports(source_practice_id);
CREATE INDEX IF NOT EXISTS plagiarism_reports_created_at_idx ON plagiarism_reports(created_at DESC);
CREATE INDEX IF NOT EXISTS plagiarism_reports_has_plagiarism_idx ON plagiarism_reports(has_plagiarism);

CREATE INDEX IF NOT EXISTS plagiarism_matches_report_id_idx ON plagiarism_matches(report_id);
CREATE INDEX IF NOT EXISTS plagiarism_matches_matched_practice_id_idx ON plagiarism_matches(matched_practice_id);
CREATE INDEX IF NOT EXISTS plagiarism_matches_similarity_score_idx ON plagiarism_matches(similarity_score DESC);