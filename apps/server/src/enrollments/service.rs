use async_trait::async_trait;
use chrono::{Duration, Utc};
use shaku::{Component, Interface};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    courses::CourseRepository,
    enrollments::*,
    practices::*,
    shared::{
        AppResult, NotFoundError, ValidationError,
        services::{Event, EventQueue},
    },
    users::*,
};

#[derive(Component)]
#[shaku(interface = EnrollmentService)]
pub struct EnrollmentServiceImpl {
    #[shaku(inject)]
    enrollments: Arc<dyn EnrollmentRepository>,

    #[shaku(inject)]
    users: Arc<dyn UserRepository>,

    #[shaku(inject)]
    courses: Arc<dyn CourseRepository>,

    #[shaku(inject)]
    practices: Arc<dyn PracticeRepository>,

    #[shaku(inject)]
    event_queue: Arc<dyn EventQueue>,
}

#[async_trait]
pub trait EnrollmentService: Interface {
    async fn get_all(
        &self,
        filter: EnrollmentFilter,
    ) -> AppResult<Vec<EnrollmentWithStudentAndPractice>>;

    async fn get_by_id(
        &self,
        id: &Uuid,
    ) -> AppResult<EnrollmentWithStudentAndPractice>;

    async fn create(&self, input: CreateEnrollmentDto) -> AppResult<Enrollment>;

    async fn update(
        &self,
        id: &Uuid,
        input: UpdateEnrollmentDto,
    ) -> AppResult<Enrollment>;

    async fn upload_final_report(
        &self,
        id: &Uuid,
        doc_bytes: Vec<u8>,
    ) -> AppResult<()>;

    async fn remove(&self, id: &Uuid) -> AppResult<()>;
}

#[async_trait]
impl EnrollmentService for EnrollmentServiceImpl {
    async fn get_all(
        &self,
        filter: EnrollmentFilter,
    ) -> AppResult<Vec<EnrollmentWithStudentAndPractice>> {
        let mut result = Vec::new();
        let enrollments = self.enrollments.find_many(filter).await?;

        let student_filter = user_filter! {
            ids: enrollments.iter().map(|e| e.student_id).collect::<Vec<_>>()
        };

        let practice_filter = practice_filter! {
            ids: enrollments.iter().filter_map(|e| e.practice_id).collect::<Vec<_>>()
        };

        let students = self.users.find_many(student_filter).await?;
        let practices = self.practices.find_many(practice_filter).await?;

        for enrollment in enrollments {
            let student = students
                .iter()
                .find(|s| s.id == enrollment.student_id)
                .cloned()
                .ok_or(NotFoundError::user(enrollment.student_id))?;

            let practice = if let Some(practice_id) = enrollment.practice_id {
                practices.iter().find(|p| p.id == practice_id).cloned()
            } else {
                None
            };

            result.push((enrollment, student, practice));
        }

        Ok(result)
    }

    async fn get_by_id(
        &self,
        id: &Uuid,
    ) -> AppResult<EnrollmentWithStudentAndPractice> {
        let enrollment = self
            .enrollments
            .find_by_id(id)
            .await?
            .ok_or(NotFoundError::enrollment(*id))?;

        let student = self
            .users
            .find_by_id(&enrollment.student_id)
            .await?
            .ok_or(NotFoundError::user(enrollment.student_id))?;

        let practice = match enrollment.practice_id {
            Some(practice_id) => self.practices.find_by_id(&practice_id).await?,
            None => None,
        };

        Ok((enrollment, student, practice))
    }

    async fn create(&self, input: CreateEnrollmentDto) -> AppResult<Enrollment> {
        let enrollment = Enrollment::from(input);

        let filter = enrollment_filter! {
            student_id: enrollment.student_id,
            course_id: enrollment.course_id,
        };

        if !self.enrollments.find_many(filter).await?.is_empty() {
            return Err(ValidationError::duplicate_enrollment(
                enrollment.student_id,
                enrollment.course_id,
            ))?;
        }

        let (student_exists, course_exists) = {
            let (student, course) = tokio::join!(
                self.users.find_by_id(&enrollment.student_id),
                self.courses.find_by_id(&enrollment.course_id)
            );

            (student.ok().flatten(), course.ok().flatten())
        };

        let Some(student) = student_exists else {
            return Err(NotFoundError::user(enrollment.student_id))?;
        };

        if course_exists.is_none() {
            return Err(NotFoundError::course(enrollment.course_id))?;
        };

        if !student.is_student() {
            return Err(ValidationError::not_a_student(student.id))?;
        }

        self.enrollments.save(enrollment).await
    }

    async fn upload_final_report(
        &self,
        id: &Uuid,
        doc_bytes: Vec<u8>,
    ) -> AppResult<()> {
        let (enrollment, student, practice) = self.get_by_id(id).await?;

        let Some(practice) = practice else {
            return Err(ValidationError::NoPracticeAssociated)?;
        };

        let course = self
            .courses
            .find_by_id(&enrollment.course_id)
            .await?
            .ok_or(NotFoundError::course(enrollment.course_id))?;

        let teacher = self
            .users
            .find_by_id(&course.teacher_id)
            .await?
            .ok_or(NotFoundError::user(course.teacher_id))?;

        let limit_upload_date = practice.end_date + Duration::days(14);

        if Utc::now() > limit_upload_date {
            return Err(ValidationError::FinalReportUploadExpired)?;
        }

        let event_data = (enrollment, course, teacher, student, doc_bytes);

        self.event_queue
            .publish(Event::FinalReportUploaded(event_data))
            .await;

        Ok(())
    }

    async fn update(
        &self,
        id: &Uuid,
        input: UpdateEnrollmentDto,
    ) -> AppResult<Enrollment> {
        let Some(mut enrollment) = self.enrollments.find_by_id(id).await? else {
            return Err(NotFoundError::enrollment(*id))?;
        };

        if let Some(scores) = input.student_scores {
            enrollment.student_scores =
                scores.into_iter().map(StudentScore::from).collect();
        }

        if let Some(practice_id) = input.practice_id {
            enrollment.practice_id = Some(Uuid::parse_str(&practice_id).unwrap());
        }

        self.enrollments.save(enrollment).await
    }

    async fn remove(&self, id: &Uuid) -> AppResult<()> {
        if self.enrollments.find_by_id(id).await?.is_none() {
            return Err(NotFoundError::enrollment(*id))?;
        };

        self.enrollments.delete(id).await
    }
}
