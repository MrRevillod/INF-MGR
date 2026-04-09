// use chrono::{Duration, Utc};
use std::sync::Arc;
use sword::core::injectable;
use uuid::Uuid;

use crate::{
    courses::*,
    enrollments::*,
    practices::*,
    shared::{
        AppResult, NotFoundError, ValidationError,
        event_queue::{Event, EventQueue},
    },
    users::*,
};

#[injectable]
pub struct EnrollmentService {
    enrollments: Arc<EnrollmentRepository>,
    users: Arc<UserRepository>,
    courses: Arc<CourseRepository>,
    practices: Arc<PracticeRepository>,
    event_queue: Arc<EventQueue>,
    report_service: Arc<PracticeReportService>,
}

impl EnrollmentService {
    pub async fn get_all(
        &self,
        filter: EnrollmentFilter,
    ) -> AppResult<Vec<EnrollmentWithStudentAndPracticeAndCourse>> {
        let mut result = Vec::new();
        let enrollments = self.enrollments.find_many(filter).await?;

        let student_filter = user_filter! {
            ids: enrollments.iter().map(|e| e.student_id).collect::<Vec<_>>()
        };

        let practice_filter = practice_filter! {
            ids: enrollments.iter().filter_map(|e| e.practice_id).collect::<Vec<_>>()
        };

        let course_filter = course_filter! {
            ids: enrollments.iter().map(|e| e.course_id).collect::<Vec<_>>()
        };

        let course = self.courses.find_many(course_filter).await?;
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

            let course = course
                .iter()
                .find(|c| c.id == enrollment.course_id)
                .cloned()
                .ok_or(NotFoundError::course(enrollment.course_id))?;

            result.push((enrollment, student, practice, course));
        }

        Ok(result)
    }

    pub async fn get_by_id(
        &self,
        id: &Uuid,
    ) -> AppResult<EnrollmentWithStudentAndPracticeAndCourse> {
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

        let course = self
            .courses
            .find_by_id(&enrollment.course_id)
            .await?
            .ok_or(NotFoundError::course(enrollment.course_id))?;

        Ok((enrollment, student, practice, course))
    }

    pub async fn create(&self, input: CreateEnrollmentDto) -> AppResult<Enrollment> {
        let mut enrollment = Enrollment::from(input);

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

        let Some(course) = course_exists else {
            return Err(NotFoundError::course(enrollment.course_id))?;
        };

        if !student.is_student() {
            return Err(ValidationError::not_a_student(student.id))?;
        }

        enrollment.student_scores = course
            .evaluations
            .iter()
            .map(|evaluation| StudentScore {
                evaluation_id: evaluation.id,
                score: 0.0,
            })
            .collect();

        self.enrollments.save(enrollment).await
    }

    pub async fn upload_final_report(
        &self,
        id: &Uuid,
        files: (Vec<u8>, Vec<u8>),
    ) -> AppResult<()> {
        let (doc_bytes, tex_project_zip_bytes) = files;
        let (enrollment, student, practice, _) = self.get_by_id(id).await?;

        let Some(practice) = practice else {
            return Err(ValidationError::NoPracticeAssociated)?;
        };

        self.report_service
            .save_zipped_project(&practice.id, tex_project_zip_bytes)
            .await?;

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

        // let Some(end_date) = practice.end_date else {
        //     return Err(ValidationError::FinalReportUploadExpired)?;
        // };

        // let limit_upload_date = end_date + Duration::days(14);

        // if Utc::now() > limit_upload_date {
        //     return Err(ValidationError::FinalReportUploadExpired)?;
        // }

        let event_data = (enrollment, course, teacher, student, doc_bytes);

        self.event_queue
            .publish(Event::FinalReportUploaded(event_data))
            .await;

        Ok(())
    }

    pub async fn update(
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

    pub async fn remove(&self, id: &Uuid) -> AppResult<()> {
        if self.enrollments.find_by_id(id).await?.is_none() {
            return Err(NotFoundError::enrollment(*id))?;
        };

        self.enrollments.delete(id).await
    }
}
