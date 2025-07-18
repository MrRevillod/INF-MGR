CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

DROP TYPE IF EXISTS student_status CASCADE;
CREATE TYPE student_status AS ENUM (
    'active',
    'inactive',
    'completed',
    'evaluating'
);

DROP TYPE IF EXISTS role CASCADE;
CREATE TYPE user_role AS ENUM (
    'administrator',
    'teacher',
    'student',
    'coordinator',
    'secretary'
);

DROP TYPE IF EXISTS evaluation CASCADE;
CREATE TYPE evaluation AS (
    id UUID,
    name TEXT,
    weight FLOAT
);

DROP TYPE IF EXISTS student_evaluation CASCADE;
CREATE TYPE student_evaluation AS (
    id UUID,
    score FLOAT
)
