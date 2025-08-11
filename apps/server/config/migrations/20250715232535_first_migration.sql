CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DROP TYPE IF EXISTS course_status CASCADE;
CREATE TYPE course_status AS ENUM (
    'active',
    'completed'
);

DROP TYPE IF EXISTS user_role CASCADE;
CREATE TYPE user_role AS ENUM (
    'administrator',
    'teacher',
    'student',
    'coordinator',
    'secretary'
);

DROP TYPE IF EXISTS course_evaluation CASCADE;
CREATE TYPE course_evaluation AS (
    id UUID,
    name TEXT,
    weight INTEGER
);

DROP TYPE IF EXISTS student_score CASCADE;
CREATE TYPE student_score AS (
    evaluation_id UUID,
    score FLOAT
);
