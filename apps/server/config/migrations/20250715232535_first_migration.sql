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
    name TEXT,
    score FLOAT,
    weight FLOAT
);