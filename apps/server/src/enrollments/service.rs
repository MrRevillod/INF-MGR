use async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;
use uuid::Uuid;

use crate::courses::CourseRepository;
use crate::enrollments::EnrollmentWithStudent;
use crate::shared::errors::{AppError, Input};
use crate::users::UserRepository;

use crate::enrollments::{
    CreateEnrollmentDto, Enrollment, EnrollmentFilter, EnrollmentRepository,
    StudentScore, UpdateEnrollmentDto,
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
    ) -> Result<Vec<EnrollmentWithStudent>, AppError>;

    async fn create(
        &self,
        input: CreateEnrollmentDto,
    ) -> Result<Enrollment, AppError>;

    async fn update(
        &self,
        id: &Uuid,
        input: UpdateEnrollmentDto,
    ) -> Result<Enrollment, AppError>;

    async fn remove(&self, id: &Uuid) -> Result<(), AppError>;
}

#[async_trait]
impl EnrollmentService for EnrollmentServiceImpl {
    async fn get_all(
        &self,
        filter: EnrollmentFilter,
    ) -> Result<Vec<EnrollmentWithStudent>, AppError> {
        let mut result = Vec::new();
        let enrollments = self.enrollments.find_all(filter).await?;

        for enrollment in enrollments {
            let Some(student) = self
                .users
                .find_by_id(&enrollment.student_id)
                .await
                .ok()
                .flatten()
            else {
                continue;
            };

            result.push((enrollment, student));
        }

        Ok(result)
    }

    async fn create(
        &self,
        input: CreateEnrollmentDto,
    ) -> Result<Enrollment, AppError> {
        let enrollment = Enrollment::from(input);

        let filter = EnrollmentFilter {
            student_id: Some(enrollment.student_id),
            course_id: Some(enrollment.course_id),
        };

        if !self.enrollments.find_all(filter).await?.is_empty() {
            return Err(AppError::Conflict(Input {
                message: "El estudiante ya está inscrito en este curso.".to_string(),
                ..Input::default()
            }));
        }

        let (student_exists, course_exists) = {
            let (student, course) = tokio::join!(
                self.users.find_by_id(&enrollment.student_id),
                self.courses.find_by_id(&enrollment.course_id)
            );

            (student.ok().flatten(), course.ok().flatten())
        };

        let Some(student) = student_exists else {
            return Err(AppError::ResourceNotFound {
                id: enrollment.student_id.to_string(),
                kind: "Student",
            });
        };

        if course_exists.is_none() {
            return Err(AppError::ResourceNotFound {
                id: enrollment.course_id.to_string(),
                kind: "Course",
            });
        };

        if !student.is_student() {
            return Err(AppError::InvalidInput(Input {
                field: "studentId".to_string(),
                message: "El usuario no es un estudiante.".to_string(),
                value: enrollment.student_id.to_string(),
            }));
        }

        self.enrollments.save(enrollment).await
    }

    async fn update(
        &self,
        id: &Uuid,
        input: UpdateEnrollmentDto,
    ) -> Result<Enrollment, AppError> {
        let Some(mut enrollment) = self.enrollments.find_by_id(id).await? else {
            return Err(AppError::ResourceNotFound {
                id: id.to_string(),
                kind: "Enrollment",
            });
        };

        if let Some(scores) = input.student_scores {
            enrollment.student_scores =
                scores.into_iter().map(StudentScore::from).collect();
        }

        self.enrollments.save(enrollment).await
    }

    async fn remove(&self, id: &Uuid) -> Result<(), AppError> {
        if self.enrollments.find_by_id(id).await?.is_none() {
            return Err(AppError::ResourceNotFound {
                id: id.to_string(),
                kind: "Enrollment",
            });
        };

        self.enrollments.delete(id).await
    }
}
