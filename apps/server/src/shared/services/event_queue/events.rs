use chrono::{DateTime, Utc};
use chrono_tz::America::Santiago;

use crate::{courses::Course, enrollments::Enrollment, practices::Practice, users::User};

#[derive(Debug, Clone)]
pub enum Event {
    PracticeCreated((User, Practice, Course, Enrollment)),
    PracticeApproved((User, Practice, Course, User)),
    PracticeDeclined((User, Practice, Course, User)),
    UserCreated((String, String, String)),
    ManyUsersCreated(Vec<(String, String, String)>),
    CourseCreated((Course, User)),
}

pub fn format_date(date: String) -> String {
    let date = DateTime::parse_from_rfc3339(&date).map(|dt| dt.with_timezone(&Utc)).ok();

    date.map(|date| date.with_timezone(&Santiago).format("%d/%m/%y").to_string())
        .unwrap_or_default()
}
