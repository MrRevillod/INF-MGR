#[macro_export]
macro_rules! filter {
    ($struct_name:ident, { $($field:ident $(: $value:expr)?),* $(,)? }) => {
        {
            let mut filter = $struct_name::default();
            $(
                filter.$field = Some($crate::filter!(@assign $field $(: $value)?));
            )*
            filter
        }
    };

    (@assign $field:ident : $value:expr) => { $value };
    (@assign $field:ident) => { $field };

    (@assign_user $filter:ident, page : $value:expr) => {
        $filter.page = $value;
    };
    (@assign_user $filter:ident, page) => {
        $filter.page = page;
    };
    (@assign_user $filter:ident, $field:ident : $value:expr) => {
        $filter.$field = Some($value);
    };
    (@assign_user $filter:ident, $field:ident) => {
        $filter.$field = Some($field);
    };
}

#[macro_export]
macro_rules! user_filter {
    ($($field:ident $(: $value:expr)?),* $(,)?) => {
        $crate::filter!(UserFilter, { $($field $(: $value)?),* })
    };
}

#[macro_export]
macro_rules! practice_filter {
    ($($field:ident $(: $value:expr)?),* $(,)?) => {
        $crate::filter!(PracticeFilter, { $($field $(: $value)?),* })
    };
}

#[macro_export]
macro_rules! enrollment_filter {
    ($($field:ident $(: $value:expr)?),* $(,)?) => {
        $crate::filter!(EnrollmentFilter, { $($field $(: $value)?),* })
    };
}

#[macro_export]
macro_rules! course_filter {
    ($($field:ident $(: $value:expr)?),* $(,)?) => {
        $crate::filter!(CourseFilter, { $($field $(: $value)?),* })
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
