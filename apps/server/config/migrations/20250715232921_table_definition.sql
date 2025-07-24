CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    rut TEXT NOT NULL,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password TEXT NOT NULL,
    roles user_role[] NOT NULL,
    deleted_at TIMESTAMP WITH TIME ZONE NULL
);

CREATE TABLE IF NOT EXISTS practices (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    enterprise_name TEXT NOT NULL,
    location TEXT NOT NULL,
    description TEXT NOT NULL,
    supervisor_name TEXT NOT NULL,
    supervisor_email TEXT NOT NULL,
    start_date TIMESTAMP WITH TIME ZONE NOT NULL,
    end_date TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE TABLE IF NOT EXISTS asignatures (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    year INTEGER NOT NULL,
    code TEXT NOT NULL,
    name TEXT NOT NULL,
    evaluations evaluation[] NOT NULL,
    teacher_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    status asignature_status NOT NULL DEFAULT 'inprogress'
);

CREATE TABLE IF NOT EXISTS inscriptions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    asignature_id UUID NOT NULL REFERENCES asignatures(id) ON DELETE RESTRICT,
    practice_id UUID REFERENCES practices(id) ON DELETE SET NULL,
    evaluations_scores student_evaluation[] NOT NULL,
    status student_status NOT NULL DEFAULT 'active',
    UNIQUE(user_id, asignature_id)
);

CREATE TABLE IF NOT EXISTS reports (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    inscription_id UUID NOT NULL REFERENCES inscriptions(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);
