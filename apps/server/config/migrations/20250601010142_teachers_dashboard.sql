-- Add migration script here

CREATE TABLE IF NOT EXISTS thesis_ideas (
    id UUID PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    mentor_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    proposer_id TEXT NOT NULL,
    proposer_name TEXT NOT NULL,
    proposer_email TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL
);

DO $$
BEGIN
  IF NOT EXISTS (
    SELECT 1 FROM pg_trigger WHERE tgname = 'update_thesis_ideas_updated_at'
  ) THEN
    CREATE TRIGGER update_thesis_ideas_updated_at
    BEFORE UPDATE ON thesis_ideas
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
  END IF;
END;
$$;