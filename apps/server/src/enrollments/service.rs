use async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;
use uuid::Uuid;

use crate::courses::CourseRepository;
use crate::enrollments::EnrollmentWithStudent;
use crate::users::UserRepository;

use crate::enrollments::{
    CreateEnrollmentDto, Enrollment, EnrollmentError, EnrollmentFilter,
    EnrollmentRepository, StudentScore, UpdateEnrollmentDto,
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
}

#[async_trait]
pub trait EnrollmentService: Interface {
    async fn get_all(
        &self,
        filter: EnrollmentFilter,
    ) -> Result<Vec<EnrollmentWithStudent>, EnrollmentError>;

    async fn create(
        &self,
        input: CreateEnrollmentDto,
    ) -> Result<Enrollment, EnrollmentError>;

    async fn update(
        &self,
        id: &Uuid,
        input: UpdateEnrollmentDto,
    ) -> Result<Enrollment, EnrollmentError>;

    async fn remove(&self, id: &Uuid) -> Result<(), EnrollmentError>;
}

#[async_trait]
impl EnrollmentService for EnrollmentServiceImpl {
    async fn get_all(
        &self,
        filter: EnrollmentFilter,
    ) -> Result<Vec<EnrollmentWithStudent>, EnrollmentError> {
        let mut result = Vec::new();
        let enrollments = self.enrollments.find_all(filter).await?;

        for enrollment in enrollments {
            let student = self
                .users
                .find_by_id(&enrollment.student_id)
                .await
                .map_err(|e| EnrollmentError::ForeignUserError(e.to_string()))?
                .ok_or(EnrollmentError::StudentNotFound)?;

            result.push((enrollment, student));
        }

        Ok(result)
    }

    async fn create(
        &self,
        input: CreateEnrollmentDto,
    ) -> Result<Enrollment, EnrollmentError> {
        let enrollment = Enrollment::from(input);

        let filter = EnrollmentFilter {
            student_id: Some(enrollment.student_id),
            course_id: Some(enrollment.course_id),
        };

        if !self.enrollments.find_all(filter).await?.is_empty() {
            return Err(EnrollmentError::InscriptionAlreadyExists);
        }

        let (student_exists, course_exists) = tokio::join!(
            self.users.find_by_id(&enrollment.student_id),
            self.courses.find_by_id(&enrollment.course_id)
        );

        let (student_exists, course_exists) = (
            student_exists
                .map_err(|e| EnrollmentError::ForeignUserError(e.to_string()))?,
            course_exists
                .map_err(|e| EnrollmentError::ForeignCourseError(e.to_string()))?,
        );

        let Some(student) = student_exists else {
            return Err(EnrollmentError::StudentNotFound);
        };

        if course_exists.is_none() {
            return Err(EnrollmentError::AsignatureNotFound);
        };

        if !student.is_student() {
            return Err(EnrollmentError::InvalidStudentRole);
        }

        self.enrollments.save(enrollment).await
    }

    async fn update(
        &self,
        id: &Uuid,
        input: UpdateEnrollmentDto,
    ) -> Result<Enrollment, EnrollmentError> {
        let Some(mut enrollment) = self.enrollments.find_by_id(id).await? else {
            return Err(EnrollmentError::NotFound);
        };

        if let Some(scores) = input.student_scores {
            enrollment.student_scores =
                scores.into_iter().map(StudentScore::from).collect();
        }

        self.enrollments.save(enrollment).await
    }

    async fn remove(&self, id: &Uuid) -> Result<(), EnrollmentError> {
        if self.enrollments.find_by_id(id).await?.is_none() {
            return Err(EnrollmentError::NotFound);
        };

        self.enrollments.delete(id).await
    }
}
