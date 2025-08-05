use async_trait::async_trait;
use chrono::Utc;
use services::mailer::{MailContext, MailTo, Mailer};
use shaku::{Component, Interface};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    courses::CourseService,
    enrollments::EnrollmentService,
    practices::{
        CreatePracticeDto, Practice, PracticeRepository, UpdatePracticeDto,
    },
    shared::errors::AppError,
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
    mailer: Arc<dyn Mailer>,
}

#[async_trait]
pub trait PracticeService: Interface {
    async fn get_by_enrollment_id(
        &self,
        id: &Uuid,
    ) -> Result<Option<Practice>, AppError>;

    async fn update(
        &self,
        id: &Uuid,
        input: UpdatePracticeDto,
    ) -> Result<Practice, AppError>;

    async fn create(&self, input: CreatePracticeDto) -> Result<Practice, AppError>;
    async fn remove(&self, id: &Uuid) -> Result<(), AppError>;
    async fn practice_exists(&self, id: &Uuid) -> Result<bool, AppError>;
}

#[async_trait]
impl PracticeService for PracticeServiceImpl {
    async fn get_by_enrollment_id(
        &self,
        enrollment_id: &Uuid,
    ) -> Result<Option<Practice>, AppError> {
        self.practices.find_by_enrollment_id(enrollment_id).await
    }

    async fn create(&self, input: CreatePracticeDto) -> Result<Practice, AppError> {
        let practice = Practice::from(input);

        let (mut enrollment, student, _) =
            self.enrollments.get_by_id(&practice.enrollment_id).await?;

        let course = self.courses.get_by_id(&enrollment.course_id).await?;

        // generate jsonwebtoken for unique 1TIME approval and rejection link
        let approval_link =
            format!("/practices/supervisor/approval/{}", practice.id);

        let rejection_link =
            format!("/practices/supervisor/rejection/{}", practice.id);

        let supervisor_email_context = MailContext::new(self.mailer.get_config())
            .insert("supervisor_name", &practice.supervisor_name)
            .insert("enterprise_name", &practice.enterprise_name)
            .insert("course_name", &course.name)
            .insert("student_name", &student.name)
            .insert("approval_link", &approval_link)
            .insert("rejection_link", &rejection_link);

        let student_email_context = MailContext::new(self.mailer.get_config())
            .insert("student_name", &student.name)
            .insert("enterprise_name", &practice.enterprise_name)
            .insert("course_name", &course.name)
            .insert("supervisor_name", &practice.supervisor_name);

        self.mailer
            .send(MailTo {
                email: practice.supervisor_email.clone(),
                subject: "Solicitud de Inscripción de Práctica",
                template: "practices:creation:supervisor",
                context: supervisor_email_context,
            })
            .await?;

        self.mailer
            .send(MailTo {
                email: student.email.clone(),
                subject: "Inscripción a Práctica Aprobada",
                template: "practices:creation:student",
                context: student_email_context,
            })
            .await?;

        let practice = self.practices.save(practice).await?;

        enrollment.practice_id = Some(practice.id);
        self.enrollments.save(enrollment).await?;

        Ok(practice)
    }

    async fn update(
        &self,
        id: &Uuid,
        input: UpdatePracticeDto,
    ) -> Result<Practice, AppError> {
        let mut practice = self.practices.find_by_id(id).await?.ok_or(
            AppError::ResourceNotFound {
                id: id.to_string(),
                kind: "Practice",
            },
        )?;

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
            practice.start_date = Some(start_date);
        }

        if let Some(end_date) = input.end_date {
            practice.end_date = Some(end_date);
        }

        self.practices.save(practice).await
    }

    async fn remove(&self, id: &Uuid) -> Result<(), AppError> {
        let practice = self.practices.find_by_id(id).await?.ok_or(
            AppError::ResourceNotFound {
                id: id.to_string(),
                kind: "Practice",
            },
        )?;

        if practice.start_date.is_some_and(|date| date < Utc::now()) {
            return Err(AppError::InvalidOperation(
                "No se puede eliminar una práctica que ya ha comenzado.".to_string(),
            ));
        }

        self.practices.delete(id).await
    }

    async fn practice_exists(&self, id: &Uuid) -> Result<bool, AppError> {
        match self.practices.find_by_id(id).await? {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}
