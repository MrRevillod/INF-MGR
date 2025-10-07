DROP TYPE IF EXISTS meeting_status CASCADE;
CREATE TYPE meeting_status AS ENUM (
    'scheduled',
    'requested'
);

CREATE TABLE IF NOT EXISTS meeting_requests (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    status meeting_status DEFAULT 'requested',
    attendees TEXT[] NOT NULL,
    course_id UUID REFERENCES courses(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);

CREATE TABLE IF NOT EXISTS meetings ( 
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    google_event_id TEXT UNIQUE,
    summary TEXT,
    description TEXT,
    start_date TIMESTAMP WITH TIME ZONE NOT NULL,
    end_date TIMESTAMP WITH TIME ZONE NOT NULL,
    attendees TEXT[] NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);