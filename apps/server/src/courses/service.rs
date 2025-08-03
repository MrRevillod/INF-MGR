use std::{str::FromStr, sync::Arc};

use async_trait::async_trait;
use shaku::{Component, Interface};
use uuid::Uuid;

use crate::{
    courses::{
        Course, CourseError, CourseFilter, CourseRepository, CourseStatus,
        CourseWithStaff, CreateCourseDto, UpdateCourseDto,
    },
    enrollments::{EnrollmentFilter, EnrollmentRepository},
    users::UserRepository,
};

#[async_trait]
pub trait CourseService: Interface {
    async fn get_all(&self) -> Result<Vec<CourseWithStaff>, CourseError>;
    async fn create(&self, input: CreateCourseDto) -> Result<Course, CourseError>;

    async fn update(
        &self,
        id: &Uuid,
        input: UpdateCourseDto,
    ) -> Result<Course, CourseError>;

    async fn delete(&self, id: &Uuid) -> Result<(), CourseError>;
}

#[derive(Component)]
#[shaku(interface = CourseService)]
pub struct CourseServiceImpl {
    #[shaku(inject)]
    courses: Arc<dyn CourseRepository>,

    #[shaku(inject)]
    users: Arc<dyn UserRepository>,

    #[shaku(inject)]
    inscriptions: Arc<dyn EnrollmentRepository>,
}

#[async_trait]
impl CourseService for CourseServiceImpl {
    async fn get_all(&self) -> Result<Vec<CourseWithStaff>, CourseError> {
        let mut result = Vec::new();
        let courses = self.courses.find(CourseFilter::default()).await?;

        for course in courses {
            let Some(teacher) = self
                .users
                .find_by_id(&course.teacher_id)
                .await
                .ok()
                .flatten()
            else {
                return Err(CourseError::TeacherNotFound);
            };

            let Some(coordinator) = self
                .users
                .find_by_id(&course.coordinator_id)
                .await
                .ok()
                .flatten()
            else {
                return Err(CourseError::CoordinatorNotFound);
            };

            result.push((course, teacher, coordinator));
        }

        Ok(result)
    }

    async fn create(&self, input: CreateCourseDto) -> Result<Course, CourseError> {
        let asignature = Course::from(input);

        let filter = CourseFilter {
            code: Some(asignature.code.clone()),
            name: Some(asignature.name.clone()),
            teacher_id: None,
            coordinator_id: None,
        };

        if !self.courses.find(filter).await?.is_empty() {
            return Err(CourseError::AlreadyExists);
        }

        let teacher_exists = self
            .users
            .find_by_id(&asignature.teacher_id)
            .await
            .map_err(|e| CourseError::ForeignUserError(e.to_string()))?;

        let Some(t) = teacher_exists else {
            return Err(CourseError::TeacherNotFound);
        };

        if !t.is_teacher() {
            return Err(CourseError::InvalidRequiredRole);
        }

        let coordinator_exists = self
            .users
            .find_by_id(&asignature.coordinator_id)
            .await
            .map_err(|e| CourseError::ForeignUserError(e.to_string()))?;

        let Some(c) = coordinator_exists else {
            return Err(CourseError::CoordinatorNotFound);
        };

        if !c.is_coordinator() {
            return Err(CourseError::InvalidRequiredRole);
        }

        Ok(self.courses.save(asignature).await?)
    }

    async fn update(
        &self,
        id: &Uuid,
        input: UpdateCourseDto,
    ) -> Result<Course, CourseError> {
        let Some(mut course) = self.courses.find_by_id(id).await? else {
            return Err(CourseError::NotFound);
        };

        if let Some(teacher_id) = input.teacher_id {
            course.teacher_id = Uuid::parse_str(&teacher_id)
                .map_err(|_| CourseError::InvalidIdentifier)?;
        }

        if let Some(coordinator_id) = input.coordinator_id {
            course.coordinator_id = Uuid::parse_str(&coordinator_id)
                .map_err(|_| CourseError::InvalidIdentifier)?;
        }

        if let Some(status) = input.status {
            course.status = CourseStatus::from_str(&status).unwrap();
        }

        let updated = self.courses.save(course).await?;

        Ok(updated)
    }

    async fn delete(&self, id: &Uuid) -> Result<(), CourseError> {
        let Some(course) = self.courses.find_by_id(id).await? else {
            return Err(CourseError::NotFound);
        };

        let filter = EnrollmentFilter {
            course_id: Some(course.id),
            ..Default::default()
        };

        let inscriptions = self
            .inscriptions
            .find_all(filter)
            .await
            .map_err(|e| CourseError::ForeignInscriptionError(e.to_string()))?;

        if !inscriptions.is_empty() {
            return Err(CourseError::HasInscriptions);
        }

        self.courses.delete(id).await
    }
}
