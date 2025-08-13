use async_trait::async_trait;
use serde_json::json;
use services::event_queue::{Event, EventQueue};

use shaku::{Component, Interface};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    courses::CourseService,
    enrollments::EnrollmentService,
    practices::{
        CreatePracticeDto, Practice, PracticeRepository, UpdatePracticeDto,
    },
    shared::{errors::AppError, AppResult},
};

#[derive(Component)]
#[shaku(interface = PracticeService)]
pub struct PracticeServiceImpl {
    #[shaku(inject)]
    practices: Arc<dyn PracticeRepository>,

    #[shaku(inject)]
    enrollments: Arc<dyn EnrollmentService>,

    #[shaku(inject)]
    courses: Arc<dyn CourseService>,

    #[shaku(inject)]
    event_queue: Arc<dyn EventQueue>,
}

#[async_trait]
pub trait PracticeService: Interface {
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Practice>, AppError>;

    async fn update(
        &self,
        id: &Uuid,
        input: UpdatePracticeDto,
    ) -> Result<Practice, AppError>;

    async fn create(
        &self,
        enrollment_id: &Uuid,
        input: CreatePracticeDto,
    ) -> Result<Practice, AppError>;

    async fn remove(&self, id: &Uuid) -> Result<(), AppError>;
    async fn practice_exists(&self, id: &Uuid) -> Result<bool, AppError>;

    async fn approve(
        &self,
        enrollment_id: &Uuid,
        practice_id: &Uuid,
    ) -> AppResult<Practice>;
}

#[async_trait]
impl PracticeService for PracticeServiceImpl {
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Practice>, AppError> {
        self.practices.find_by_id(id).await
    }

    async fn create(
        &self,
        enrollment_id: &Uuid,
        input: CreatePracticeDto,
    ) -> Result<Practice, AppError> {
        let practice = Practice::from(input);

        let (mut enrollment, student, _) =
            self.enrollments.get_by_id(&enrollment_id).await?;

        let (course, _) = self.courses.get_by_id(&enrollment.course_id).await?;

        let practice = self.practices.save(practice).await?;

        enrollment.practice_id = Some(practice.id);
        let enrollment = self.enrollments.save(enrollment).await?;

        let event_data = json!({
            "student": student,
            "practice": practice,
            "course": course,
            "enrollment": enrollment,
        });

        self.event_queue
            .publish(Event::PracticeCreated(event_data))
            .await;

        Ok(practice)
    }

    async fn approve(
        &self,
        enrollment_id: &Uuid,
        practice_id: &Uuid,
    ) -> AppResult<Practice> {
        let (enrollment, student, practice) =
            self.enrollments.get_by_id(enrollment_id).await?;

        let mut practice =
            practice.ok_or(AppError::ResourceNotFound(*practice_id))?;

        if practice.id != *practice_id {
            return Err(AppError::ResourceNotFound(*practice_id));
        }

        let (course, teacher) =
            self.courses.get_by_id(&enrollment.course_id).await?;

        let event_data = json!({
            "student": student,
            "practice": practice,
            "course": course,
            "teacher": teacher,
        });

        self.event_queue
            .publish(Event::PracticeApproved(event_data))
            .await;

        practice.is_approved = true;
        self.practices.save(practice).await
    }

    async fn update(
        &self,
        id: &Uuid,
        input: UpdatePracticeDto,
    ) -> Result<Practice, AppError> {
        let mut practice = self
            .practices
            .find_by_id(id)
            .await?
            .ok_or(AppError::ResourceNotFound(*id))?;

        if let Some(enterprise_name) = input.enterprise_name {
            practice.enterprise_name = enterprise_name;
        }

        if let Some(description) = input.description {
            practice.description = description;
        }

        if let Some(location) = input.location {
            practice.location = location;
        }

        if let Some(supervisor_name) = input.supervisor_name {
            practice.supervisor_name = supervisor_name;
        }

        if let Some(supervisor_email) = input.supervisor_email {
            practice.supervisor_email = supervisor_email;
        }

        if let Some(start_date) = input.start_date {
            practice.start_date = start_date;
        }

        if let Some(end_date) = input.end_date {
            practice.end_date = end_date;
        }

        self.practices.save(practice).await
    }

    async fn remove(&self, id: &Uuid) -> Result<(), AppError> {
        let practice = self
            .practices
            .find_by_id(id)
            .await?
            .ok_or(AppError::ResourceNotFound(*id))?;

        self.practices.delete(id).await
    }

    async fn practice_exists(&self, id: &Uuid) -> Result<bool, AppError> {
        match self.practices.find_by_id(id).await? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}
