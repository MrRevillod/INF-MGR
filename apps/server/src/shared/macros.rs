#[macro_export]
macro_rules! user_filter {
    ($($field:ident $(: $value:expr)?),* $(,)?) => {
        ::filterstruct::filter!(UserFilter, { $($field $(: $value)?),* })
    };
}

#[macro_export]
macro_rules! practice_filter {
    ($($field:ident $(: $value:expr)?),* $(,)?) => {
        ::filterstruct::filter!(PracticeFilter, { $($field $(: $value)?),* })
    };
}

#[macro_export]
macro_rules! enrollment_filter {
    ($($field:ident $(: $value:expr)?),* $(,)?) => {
        ::filterstruct::filter!(EnrollmentFilter, { $($field $(: $value)?),* })
    };
}

#[macro_export]
macro_rules! course_filter {
    ($($field:ident $(: $value:expr)?),* $(,)?) => {
        ::filterstruct::filter!(CourseFilter, { $($field $(: $value)?),* })
    };
}

#[macro_export]
macro_rules! meeting_filter {
    ($($field:ident $(: $value:expr)?),* $(,)?) => {
        ::filterstruct::filter!(MeetingFilter, { $($field $(: $value)?),* })
    };
}

#[macro_export]
macro_rules! meeting_req_filter {
    ($($field:ident $(: $value:expr)?),* $(,)?) => {
        ::filterstruct::filter!(MeetingReqFilter, { $($field $(: $value)?),* })
    };
}

#[macro_export]
macro_rules! template_ctx {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let context: RawContext = vec![
                $(($key, $value)),*
            ];

            context
        }
    };
}
