CREATE OR REPLACE FUNCTION save_user(user_json jsonb) RETURNS users AS $$
DECLARE
    user_data jsonb := user_json;
    result users;
BEGIN
    INSERT INTO users (id, rut, name, email, google_id, role, register, deleted_at, created_at)
    VALUES (
        (user_data->>'id')::uuid,
        user_data->>'rut',
        user_data->>'name',
        user_data->>'email',
        CASE WHEN user_data->>'google_id' IS NULL OR user_data->>'google_id' = 'null' THEN NULL ELSE user_data->>'google_id' END,
        (user_data->>'role')::user_role,
        CASE WHEN user_data->>'register' IS NULL OR user_data->>'register' = 'null' THEN NULL ELSE user_data->>'register' END,
        CASE WHEN user_data->>'deleted_at' IS NULL OR user_data->>'deleted_at' = 'null' THEN NULL ELSE (user_data->>'deleted_at')::timestamptz END,
        COALESCE((user_data->>'created_at')::timestamptz, NOW())
    )
    ON CONFLICT (id)
    DO UPDATE SET
        rut = EXCLUDED.rut,
        name = EXCLUDED.name,
        email = EXCLUDED.email,
        google_id = EXCLUDED.google_id,
        role = EXCLUDED.role,
        register = EXCLUDED.register
    WHERE users.deleted_at IS NULL
    RETURNING * INTO result;
    RETURN result;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION save_practice(practice_json jsonb) RETURNS practices AS $$
DECLARE
    practice_data jsonb := practice_json;
    result practices;
BEGIN
    INSERT INTO practices (id, enterprise_name, location, description, supervisor_name, supervisor_email, supervisor_phone, start_date, end_date, practice_status)
    VALUES (
        (practice_data->>'id')::uuid,
        practice_data->>'enterprise_name',
        practice_data->>'location',
        practice_data->>'description',
        practice_data->>'supervisor_name',
        practice_data->>'supervisor_email',
        practice_data->>'supervisor_phone',
        CASE WHEN practice_data->>'start_date' IS NULL OR practice_data->>'start_date' = 'null' THEN NULL ELSE (practice_data->>'start_date')::timestamptz END,
        CASE WHEN practice_data->>'end_date' IS NULL OR practice_data->>'end_date' = 'null' THEN NULL ELSE (practice_data->>'end_date')::timestamptz END,
        (practice_data->>'practice_status')::practice_status
    )
    ON CONFLICT (id) DO UPDATE SET
        enterprise_name = EXCLUDED.enterprise_name,
        location = EXCLUDED.location,
        description = EXCLUDED.description,
        supervisor_name = EXCLUDED.supervisor_name,
        supervisor_email = EXCLUDED.supervisor_email,
        supervisor_phone = EXCLUDED.supervisor_phone,
        start_date = EXCLUDED.start_date,
        end_date = EXCLUDED.end_date,
        practice_status = EXCLUDED.practice_status
    RETURNING * INTO result;
    RETURN result;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION save_course(course_json jsonb) RETURNS courses AS $$
DECLARE
    course_data jsonb := course_json;
    result courses;
BEGIN
    INSERT INTO courses (id, year, code, name, course_status, evaluations, teacher_id)
    VALUES (
        (course_data->>'id')::uuid,
        (course_data->>'year')::integer,
        course_data->>'code',
        course_data->>'name',
        (course_data->>'course_status')::course_status,
        CASE 
            WHEN course_data->>'evaluations' IS NULL OR course_data->>'evaluations' = 'null' THEN ARRAY[]::course_evaluation[]
            WHEN jsonb_typeof(course_data->'evaluations') = 'array' THEN 
                (course_data->'evaluations')::course_evaluation[]
            ELSE ARRAY[]::course_evaluation[]
        END,
        (course_data->>'teacher_id')::uuid
    )
    ON CONFLICT (id) DO UPDATE SET
        course_status = EXCLUDED.course_status,
        evaluations = EXCLUDED.evaluations,
        teacher_id = EXCLUDED.teacher_id
    RETURNING * INTO result;
    RETURN result;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION save_enrollment(enrollment_json jsonb) RETURNS enrollments AS $$
DECLARE
    enrollment_data jsonb := enrollment_json;
    result enrollments;
BEGIN
    INSERT INTO enrollments (id, student_id, course_id, practice_id, student_scores)
    VALUES (
        (enrollment_data->>'id')::uuid,
        (enrollment_data->>'student_id')::uuid,
        (enrollment_data->>'course_id')::uuid,
        CASE WHEN enrollment_data->>'practice_id' IS NULL THEN NULL ELSE (enrollment_data->>'practice_id')::uuid END,
        CASE 
            WHEN enrollment_data->>'student_scores' IS NULL OR enrollment_data->>'student_scores' = 'null' THEN ARRAY[]::student_score[]
            WHEN jsonb_typeof(enrollment_data->'student_scores') = 'array' THEN 
                (enrollment_data->'student_scores')::student_score[]
            ELSE ARRAY[]::student_score[]
        END
    )
    ON CONFLICT (id) DO UPDATE SET
        practice_id = EXCLUDED.practice_id,
        student_scores = EXCLUDED.student_scores
    RETURNING * INTO result;
    RETURN result;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION create_users(users_json jsonb[]) RETURNS SETOF users AS $$
BEGIN
    RETURN QUERY
    INSERT INTO users (id, rut, name, email, google_id, role, register, deleted_at, created_at)
    SELECT
        (user_data->>'id')::uuid,
        user_data->>'rut',
        user_data->>'name',
        user_data->>'email',
        CASE WHEN user_data->>'google_id' IS NULL OR user_data->>'google_id' = 'null' THEN NULL ELSE user_data->>'google_id' END,
        (user_data->>'role')::user_role,
        CASE WHEN user_data->>'register' IS NULL OR user_data->>'register' = 'null' THEN NULL ELSE user_data->>'register' END,
        CASE WHEN user_data->>'deleted_at' IS NULL OR user_data->>'deleted_at' = 'null' THEN NULL ELSE (user_data->>'deleted_at')::timestamptz END,
        COALESCE((user_data->>'created_at')::timestamptz, NOW())
    FROM unnest(users_json) AS user_data
    ON CONFLICT (id) DO NOTHING
    RETURNING *;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION save_meeting(meeting_json jsonb) RETURNS meetings AS $$
DECLARE
    meeting_data jsonb := meeting_json;
    result meetings;
BEGIN
    INSERT INTO meetings (id, google_event_id, summary, description, start_date, end_date, attendees, created_at, updated_at)
    VALUES (
        (meeting_data->>'id')::uuid,
        CASE WHEN meeting_data->>'google_event_id' IS NULL OR meeting_data->>'google_event_id' = 'null' THEN NULL ELSE meeting_data->>'google_event_id' END,
        CASE WHEN meeting_data->>'summary' IS NULL OR meeting_data->>'summary' = 'null' THEN NULL ELSE meeting_data->>'summary' END,
        CASE WHEN meeting_data->>'description' IS NULL OR meeting_data->>'description' = 'null' THEN NULL ELSE meeting_data->>'description' END,
        (meeting_data->>'start_date')::timestamptz,
        (meeting_data->>'end_date')::timestamptz,
        CASE 
            WHEN meeting_data->>'attendees' IS NULL OR meeting_data->>'attendees' = 'null' THEN ARRAY[]::TEXT[]
            WHEN jsonb_typeof(meeting_data->'attendees') = 'array' THEN 
                ARRAY(SELECT jsonb_array_elements_text(meeting_data->'attendees'))::TEXT[]
            ELSE ARRAY[]::TEXT[]
        END,
        COALESCE((meeting_data->>'created_at')::timestamptz, NOW()),
        COALESCE((meeting_data->>'updated_at')::timestamptz, NOW())
    )
    ON CONFLICT (id) DO UPDATE SET
        google_event_id = EXCLUDED.google_event_id,
        summary = EXCLUDED.summary,
        description = EXCLUDED.description,
        start_date = EXCLUDED.start_date,
        end_date = EXCLUDED.end_date,
        attendees = EXCLUDED.attendees,
        updated_at = EXCLUDED.updated_at
    RETURNING * INTO result;
    RETURN result;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION save_meeting_request(meeting_req_json jsonb) RETURNS meeting_requests AS $$
DECLARE
    meeting_req_data jsonb := meeting_req_json;
    result meeting_requests;
BEGIN
    INSERT INTO meeting_requests (id, status, attendees, course_id, created_at, updated_at)
    VALUES (
        (meeting_req_data->>'id')::uuid,
        (meeting_req_data->>'status')::text::meeting_status,
        CASE 
            WHEN meeting_req_data->>'attendees' IS NULL OR meeting_req_data->>'attendees' = 'null' THEN ARRAY[]::TEXT[]
            WHEN jsonb_typeof(meeting_req_data->'attendees') = 'array' THEN 
                ARRAY(SELECT jsonb_array_elements_text(meeting_req_data->'attendees'))::TEXT[]
            ELSE ARRAY[]::TEXT[]
        END,
        (meeting_req_data->>'course_id')::uuid,
        COALESCE((meeting_req_data->>'created_at')::timestamptz, NOW()),
        COALESCE((meeting_req_data->>'updated_at')::timestamptz, NOW())
    )
    ON CONFLICT (id) DO UPDATE SET
        status = EXCLUDED.status,
        attendees = EXCLUDED.attendees,
        course_id = EXCLUDED.course_id,
        updated_at = EXCLUDED.updated_at
    RETURNING * INTO result;
    RETURN result;
END;
$$ LANGUAGE plpgsql;