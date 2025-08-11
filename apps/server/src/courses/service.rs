use async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    courses::{
        Course, CourseFilter, CourseRepository, CourseWithStaff, CreateCourseDto,
        UpdateCourseDto,
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
    async fn get_by_id(&self, id: &Uuid) -> Result<CourseWithStaff, AppError>;

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
        let courses = self.courses.find(CourseFilter::default()).await?;

        let teacher_ids = courses.iter().map(|c| c.teacher_id).collect::<Vec<_>>();
        let teachers = self.users.find_by_ids(&teacher_ids).await?;

        let mut result = vec![];

        for couse in courses {
            for teacher in &teachers {
                if couse.teacher_id == teacher.id {
                    result.push((couse.clone(), teacher.clone()));
                    break;
                }
            }
        }

        Ok(result)
    }

    async fn get_by_id(&self, id: &Uuid) -> Result<CourseWithStaff, AppError> {
        let Some(course) = self.courses.find_by_id(id).await? else {
            return Err(AppError::ResourceNotFound(*id));
        };

        let teacher = self
            .users
            .find_by_id(&course.teacher_id)
            .await?
            .ok_or(AppError::ResourceNotFound(course.teacher_id))?;

        Ok((course, teacher))
    }

    async fn create(&self, input: CreateCourseDto) -> Result<Course, AppError> {
        let course = Course::from(input);

        let filter = CourseFilter {
            code: Some(course.code.clone()),
            name: Some(course.name.clone()),
            year: Some(course.year),
            teacher_id: None,
        };

        if !self.courses.find(filter).await?.is_empty() {
            return Err(AppError::Conflict(Input {
                message: "Ya existe un curso con el mismo código o nombre y año"
                    .to_string(),
                ..Default::default()
            }));
        }

        let Some(teacher) = self.users.find_by_id(&course.teacher_id).await? else {
            return Err(AppError::ResourceNotFound(course.teacher_id));
        };

        if !teacher.is_teacher() {
            return Err(AppError::InvalidInput(Input {
                field: "teacherId".to_string(),
                message: "El usuario no es un profesor".to_string(),
                value: course.teacher_id.to_string(),
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
            return Err(AppError::ResourceNotFound(*id));
        };

        if let Some(teacher_id) = input.teacher_id {
            course.teacher_id = Uuid::parse_str(&teacher_id).unwrap();
        }

        Ok(self.courses.save(course).await?)
    }

    async fn remove(&self, id: &Uuid) -> Result<(), AppError> {
        let Some(course) = self.courses.find_by_id(id).await? else {
            return Err(AppError::ResourceNotFound(*id));
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
