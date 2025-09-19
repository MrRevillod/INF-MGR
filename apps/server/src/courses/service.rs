use async_trait::async_trait;
use shaku::{Component, Interface};
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

use crate::{
    courses::*,
    enrollments::*,
    shared::{
        AppResult,
        errors::{AppError, AuthError, Input},
        services::{Event, EventQueue},
    },
    users::*,
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

    #[shaku(inject)]
    event_queue: Arc<dyn EventQueue>,
}

#[async_trait]
pub trait CourseService: Interface {
    async fn get_all(&self, filter: CourseFilter)
    -> AppResult<Vec<CourseWithStaff>>;

    async fn get_by_id(&self, id: &Uuid) -> AppResult<CourseWithStaff>;
    async fn create(&self, input: CreateCourseDto) -> AppResult<Course>;
    async fn remove(&self, id: &Uuid) -> AppResult<()>;
    async fn update(&self, id: &Uuid, input: UpdateCourseDto) -> AppResult<Course>;

    async fn check_is_teacher_course(
        &self,
        course_id: &Uuid,
        user_id: &Uuid,
    ) -> AppResult<bool>;
}

#[async_trait]
impl CourseService for CourseServiceImpl {
    async fn get_all(
        &self,
        filter: CourseFilter,
    ) -> AppResult<Vec<CourseWithStaff>> {
        let courses = self.courses.find_many(filter).await?;
        let teacher_ids = courses.iter().map(|c| c.teacher_id).collect::<Vec<_>>();

        let teachers = self
            .users
            .find_many(user_filter! { ids: teacher_ids })
            .await?;

        let teachers_map: HashMap<Uuid, &User> =
            teachers.iter().map(|t| (t.id, t)).collect();

        let mut result = vec![];

        for course in courses {
            if let Some(teacher) = teachers_map.get(&course.teacher_id) {
                result.push((course, (*teacher).clone()));
            }
        }

        Ok(result)
    }

    async fn get_by_id(&self, id: &Uuid) -> AppResult<CourseWithStaff> {
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

    async fn create(&self, input: CreateCourseDto) -> AppResult<Course> {
        let course = Course::from(input);

        let filter = course_filter! {
            code: course.code.clone(),
            name: course.name.clone(),
            year: course.year,
        };

        if !self.courses.find_many(filter).await?.is_empty() {
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

        let event_data = (course.clone(), teacher.clone());

        self.event_queue
            .publish(Event::CourseCreated(event_data))
            .await;

        Ok(self.courses.save(course).await?)
    }

    async fn update(&self, id: &Uuid, input: UpdateCourseDto) -> AppResult<Course> {
        let Some(mut course) = self.courses.find_by_id(id).await? else {
            return Err(AppError::ResourceNotFound(*id));
        };

        if let Some(teacher_id) = input.teacher_id {
            course.teacher_id = Uuid::parse_str(&teacher_id).unwrap();
        }

        if let Some(evaluation) = input.evaluations {
            course.evaluations =
                evaluation.into_iter().map(CourseEvaluation::from).collect();
        }
        Ok(self.courses.save(course).await?)
    }

    async fn remove(&self, id: &Uuid) -> Result<(), AppError> {
        let Some(course) = self.courses.find_by_id(id).await? else {
            return Err(AppError::ResourceNotFound(*id));
        };

        let filter = enrollment_filter! {
            course_id: course.id,
        };

        if !self.enrollments.find_many(filter).await?.is_empty() {
            return Err(AppError::InvalidInput(Input {
                field: "courseId".to_string(),
                message: "No se puede eliminar un curso con inscripciones activas"
                    .to_string(),
                value: course.id.to_string(),
            }));
        }

        self.courses.delete(id).await
    }

    async fn check_is_teacher_course(
        &self,
        course_id: &Uuid,
        user_id: &Uuid,
    ) -> AppResult<bool> {
        let (course, _) = self.get_by_id(course_id).await?;

        if &course.teacher_id == user_id {
            return Ok(true);
        }

        Err(AppError::from(AuthError::Other(
            "El usuario no es el profesor de este curso".to_string(),
        )))
    }
}
