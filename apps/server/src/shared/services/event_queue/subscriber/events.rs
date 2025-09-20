use crate::{
    courses::Course, enrollments::Enrollment, practices::Practice, users::User,
};

#[derive(Debug, Clone)]
pub enum Event {
    PracticeCreated(PracticeCreatedEvent),
    PracticeApproved(PracticeApprovedEvent),
    PracticeDeclined(PracticeDeclinedEvent),
    PracticeAuthorized(PracticeAuthorizedEvent),
    PracticeEvaluated(PracticeEvaluatedEvent),
    UserCreated(UserCreatedEvent),
    ManyUsersCreated(ManyUsersCreatedEvent),
    CourseCreated(CourseCreatedEvent),
}

pub type PracticeCreatedEvent = (User, Practice, Course, Enrollment);
pub type PracticeApprovedEvent = (User, Enrollment, Practice, Course, User);
pub type PracticeDeclinedEvent = (User, Enrollment, Practice, Course, User);
pub type PracticeAuthorizedEvent = (Practice, Vec<u8>);
pub type PracticeEvaluatedEvent = (User, Practice, Course, User, f64);
pub type UserCreatedEvent = (String, String);
pub type ManyUsersCreatedEvent = Vec<(String, String)>;
pub type CourseCreatedEvent = (Course, User);
