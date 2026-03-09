use crate::{
    courses::Course,
    enrollments::Enrollment,
    imports::ImportedStudent,
    practices::Practice,
    users::{Student, Teacher, User},
};

#[derive(Debug, Clone)]
pub enum Event {
    PracticeCreated(PracticeCreatedEvent),
    PracticeApproved(PracticeApprovedEvent),
    PracticeDeclined(PracticeDeclinedEvent),
    PracticeAuthorized(PracticeAuthorizedEvent),
    PracticeEvaluated(PracticeEvaluatedEvent),
    UserCreated(UserCreatedEvent),
    ImportedStudents(Vec<ImportedStudent>),
    CourseCreated(CourseCreatedEvent),
    FinalReportUploaded(FinalReportUploadedEvent),
    MeetingRequestCreated(MeetingRequestCreatedEvent),
}

pub type PracticeCreatedEvent = (User, Practice, Course, Enrollment);
pub type PracticeApprovedEvent = (User, Enrollment, Practice, Course, User);
pub type PracticeDeclinedEvent = (User, Enrollment, Practice, Course, User);
pub type PracticeAuthorizedEvent = (Student, Course, Teacher, Practice, Vec<u8>);
pub type PracticeEvaluatedEvent = (User, Practice, Course, User, f64);
pub type UserCreatedEvent = (String, String);
pub type CourseCreatedEvent = (Course, User);
pub type FinalReportUploadedEvent = (Enrollment, Course, User, User, Vec<u8>);
pub type MeetingRequestCreatedEvent = (Teacher, Course, Vec<Student>);
