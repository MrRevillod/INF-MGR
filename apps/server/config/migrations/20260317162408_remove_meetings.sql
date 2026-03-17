-- Remove meetings system tables and types

DROP TABLE IF EXISTS meetings CASCADE;
DROP TABLE IF EXISTS meeting_requests CASCADE;
DROP TYPE IF EXISTS meeting_status CASCADE;
