CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    rut TEXT NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    roles user_role[] NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE NULL
);

CREATE INDEX IF NOT EXISTS users_rut_idx ON users(rut);
CREATE INDEX IF NOT EXISTS users_roles_deleted_at_idx ON users USING GIN(roles) WHERE deleted_at IS NULL;
CREATE INDEX IF NOT EXISTS users_created_at_idx ON users(created_at DESC);

CREATE TABLE IF NOT EXISTS courses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    year INTEGER NOT NULL,
    code TEXT NOT NULL,
    name TEXT NOT NULL,
    evaluations course_evaluation[] NOT NULL,
    teacher_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    course_status course_status NOT NULL DEFAULT 'active'
);

CREATE INDEX IF NOT EXISTS courses_teacher_id_idx ON courses(teacher_id);
CREATE INDEX IF NOT EXISTS courses_year_status_idx ON courses(year, course_status);
CREATE INDEX IF NOT EXISTS courses_code_idx ON courses(code);

CREATE TABLE IF NOT EXISTS practices (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    enterprise_name TEXT NOT NULL,
    location TEXT NOT NULL,
    description TEXT NOT NULL,

    supervisor_name TEXT NOT NULL,
    supervisor_email TEXT NOT NULL,
    supervisor_phone TEXT NOT NULL,

    start_date TIMESTAMP WITH TIME ZONE,
    end_date TIMESTAMP WITH TIME ZONE,

    is_approved BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS enrollments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    student_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    course_id UUID NOT NULL REFERENCES courses(id) ON DELETE RESTRICT,
    practice_id UUID REFERENCES practices(id) ON DELETE SET NULL,
    student_scores student_score[] NOT NULL,

    UNIQUE(student_id, course_id)
);

CREATE INDEX IF NOT EXISTS enrollments_student_id_idx ON enrollments(student_id);
CREATE INDEX IF NOT EXISTS enrollments_course_id_idx ON enrollments(course_id);
CREATE INDEX IF NOT EXISTS enrollments_practice_id_idx ON enrollments(practice_id) WHERE practice_id IS NOT NULL;

