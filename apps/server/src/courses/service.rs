use async_trait::async_trait;
use shaku::{Component, Interface};
use std::{collections::HashMap, sync::Arc};
use uuid::Uuid;

use crate::{
    courses::*,
    enrollments::*,
    shared::{
        AppResult, NotFoundError, ValidationError,
        errors::{AppError, AuthError},
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
            return Err(NotFoundError::course(*id))?;
        };

        let teacher = self
            .users
            .find_by_id(&course.teacher_id)
            .await?
            .ok_or(NotFoundError::user(course.teacher_id))?;

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
            return Err(ValidationError::duplicate_course(
                &course.name,
                course.year,
                &course.code,
            ))?;
        }

        let Some(teacher) = self.users.find_by_id(&course.teacher_id).await? else {
            return Err(NotFoundError::user(course.teacher_id))?;
        };

        if !teacher.is_teacher() {
            return Err(ValidationError::not_a_teacher(teacher.id))?;
        }

        let event_data = (course.clone(), teacher.clone());

        self.event_queue
            .publish(Event::CourseCreated(event_data))
            .await;

        Ok(self.courses.save(course).await?)
    }

    async fn update(&self, id: &Uuid, input: UpdateCourseDto) -> AppResult<Course> {
        let Some(mut course) = self.courses.find_by_id(id).await? else {
            return Err(NotFoundError::course(*id))?;
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
            return Err(NotFoundError::course(*id))?;
        };

        let filter = enrollment_filter! {
            course_id: course.id,
        };

        if !self.enrollments.find_many(filter).await?.is_empty() {
            return Err(ValidationError::course_has_enrollments(course.id))?;
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
