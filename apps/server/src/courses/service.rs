use std::{str::FromStr, sync::Arc};

use async_trait::async_trait;
use shaku::{Component, Interface};
use uuid::Uuid;

use crate::{
    courses::{
        Course, CourseFilter, CourseRepository, CourseStatus, CourseWithStaff,
        CreateCourseDto, UpdateCourseDto,
    },
    enrollments::{EnrollmentFilter, EnrollmentRepository},
    shared::errors::{AppError, Input},
    users::UserRepository,
};

#[derive(Component)]
#[shaku(interface = CourseService)]
pub struct CourseServiceImpl {
    #[shaku(inject)]
    courses: Arc<dyn CourseRepository>,

    #[shaku(inject)]
    users: Arc<dyn UserRepository>,

    #[shaku(inject)]
    enrollments: Arc<dyn EnrollmentRepository>,
}

#[async_trait]
pub trait CourseService: Interface {
    async fn get_all(&self) -> Result<Vec<CourseWithStaff>, AppError>;
    async fn get_by_id(&self, id: &Uuid) -> Result<Course, AppError>;

    async fn create(&self, input: CreateCourseDto) -> Result<Course, AppError>;
    async fn remove(&self, id: &Uuid) -> Result<(), AppError>;

    async fn update(
        &self,
        id: &Uuid,
        input: UpdateCourseDto,
    ) -> Result<Course, AppError>;
}

#[async_trait]
impl CourseService for CourseServiceImpl {
    async fn get_all(&self) -> Result<Vec<CourseWithStaff>, AppError> {
        let mut result = Vec::new();
        let courses = self.courses.find(CourseFilter::default()).await?;

        for course in courses {
            let (teacher_exists, coordinator_exists) = tokio::join!(
                self.users.find_by_id(&course.teacher_id),
                self.users.find_by_id(&course.coordinator_id)
            );

            let Some(teacher) = teacher_exists? else {
                return Err(AppError::ResourceNotFound {
                    id: course.teacher_id.to_string(),
                    kind: "Teacher",
                });
            };

            let Some(coordinator) = coordinator_exists? else {
                return Err(AppError::ResourceNotFound {
                    id: course.coordinator_id.to_string(),
                    kind: "Coordinator",
                });
            };

            result.push((course, teacher, coordinator));
        }

        Ok(result)
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<Course, AppError> {
        let Some(course) = self.courses.find_by_id(id).await? else {
            return Err(AppError::ResourceNotFound {
                id: id.to_string(),
                kind: "Course",
            });
        };

        Ok(course)
    }

    async fn create(&self, input: CreateCourseDto) -> Result<Course, AppError> {
        let course = Course::from(input);

        let filter = CourseFilter {
            code: Some(course.code.clone()),
            name: Some(course.name.clone()),
            year: Some(course.year),
            teacher_id: None,
            coordinator_id: None,
        };

        if !self.courses.find(filter).await?.is_empty() {
            return Err(AppError::Conflict(Input {
                message: "Ya existe un curso con el mismo código o nombre y año"
                    .to_string(),
                ..Default::default()
            }));
        }

        let Some(teacher) = self.users.find_by_id(&course.teacher_id).await? else {
            return Err(AppError::ResourceNotFound {
                id: course.teacher_id.to_string(),
                kind: "Teacher",
            });
        };

        if !teacher.is_teacher() {
            return Err(AppError::InvalidInput(Input {
                field: "teacherId".to_string(),
                message: "El usuario no es un profesor".to_string(),
                value: course.teacher_id.to_string(),
            }));
        }

        let Some(c) = self.users.find_by_id(&course.coordinator_id).await? else {
            return Err(AppError::ResourceNotFound {
                id: course.coordinator_id.to_string(),
                kind: "Coordinator",
            });
        };

        if !c.is_coordinator() {
            return Err(AppError::InvalidInput(Input {
                field: "coordinatorId".to_string(),
                message: "El usuario no es un coordinador".to_string(),
                value: course.coordinator_id.to_string(),
            }));
        }

        Ok(self.courses.save(course).await?)
    }

    async fn update(
        &self,
        id: &Uuid,
        input: UpdateCourseDto,
    ) -> Result<Course, AppError> {
        let Some(mut course) = self.courses.find_by_id(id).await? else {
            return Err(AppError::ResourceNotFound {
                id: id.to_string(),
                kind: "Course",
            });
        };

        if let Some(teacher_id) = input.teacher_id {
            course.teacher_id = Uuid::parse_str(&teacher_id).unwrap();
        }

        if let Some(coordinator_id) = input.coordinator_id {
            course.coordinator_id = Uuid::parse_str(&coordinator_id).unwrap()
        }

        if let Some(status) = input.status {
            course.status = CourseStatus::from_str(&status).unwrap();
        }

        Ok(self.courses.save(course).await?)
    }

    async fn remove(&self, id: &Uuid) -> Result<(), AppError> {
        let Some(course) = self.courses.find_by_id(id).await? else {
            return Err(AppError::ResourceNotFound {
                id: id.to_string(),
                kind: "Course",
            });
        };

        let filter = EnrollmentFilter {
            course_id: Some(course.id),
            ..Default::default()
        };

        if !self.enrollments.find_all(filter).await?.is_empty() {
            return Err(AppError::InvalidInput(Input {
                field: "courseId".to_string(),
                message: "No se puede eliminar un curso con inscripciones activas"
                    .to_string(),
                value: course.id.to_string(),
            }));
        }

        self.courses.delete(id).await
    }
}
