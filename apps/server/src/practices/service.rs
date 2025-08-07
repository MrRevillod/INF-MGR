use async_trait::async_trait;
use chrono::Utc;
use services::{
    mailer::{MailTo, Mailer},
    printer::{PrintOptions, Printer},
    templates::RawContext,
};

use shaku::{Component, Interface};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    courses::CourseService,
    enrollments::EnrollmentService,
    practices::{
        CreatePracticeDto, Practice, PracticeRepository, UpdatePracticeDto,
    },
    shared::{errors::AppError, format_date, AppResult},
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

    #[shaku(inject)]
    printer: Arc<dyn Printer>,
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
    ) -> AppResult<String>;
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

        let (course, _, _) = self.courses.get_by_id(&enrollment.course_id).await?;

        let (start_date, end_date) = (
            practice.start_date.map(|d| d.to_string()),
            practice.end_date.map(|d| d.to_string()),
        );

        let email_context: RawContext = vec![
            ("student_name", student.name.clone()),
            ("student_email", student.email.clone()),
            ("enterprise_name", practice.enterprise_name.clone()),
            ("supervisor_name", practice.supervisor_name.clone()),
            ("supervisor_email", practice.supervisor_email.clone()),
            ("course_name", course.name.clone()),
            ("course_code", course.code.clone()),
            ("location", practice.location.clone()),
            ("start_date", start_date.unwrap_or_default()),
            ("end_date", end_date.unwrap_or_default()),
            (
                "approval_link",
                format!(
                    "/enrollments/{}/practice/{}/approve",
                    enrollment.id, practice.id
                ),
            ),
            (
                "rejection_link",
                format!(
                    "/enrollments/{}/practice/{}/reject",
                    enrollment.id, practice.id
                ),
            ),
        ];

        tokio::try_join!(
            self.mailer.send(MailTo {
                email: practice.supervisor_email.clone(),
                subject: "Solicitud de Inscripción de Práctica",
                template: "practice:creation:supervisor",
                context: email_context.clone(),
            }),
            self.mailer.send(MailTo {
                email: student.email.clone(),
                subject: "Inscripción a Práctica Aprobada",
                template: "practice:creation:student",
                context: email_context,
            })
        )?;

        let practice = self.practices.save(practice).await?;

        enrollment.practice_id = Some(practice.id);
        self.enrollments.save(enrollment).await?;

        Ok(practice)
    }

    async fn approve(
        &self,
        enrollment_id: &Uuid,
        practice_id: &Uuid,
    ) -> AppResult<String> {
        let (enrollment, student, practice) =
            self.enrollments.get_by_id(enrollment_id).await?;

        let mut practice = practice.ok_or(AppError::ResourceNotFound {
            id: practice_id.to_string(),
            kind: "Practice",
        })?;

        let (course, _, coordinator) =
            self.courses.get_by_id(&enrollment.course_id).await?;

        let enterprise_auth_pdf_ctx: RawContext = vec![
            ("student_name", student.name),
            ("course_name", course.name),
            ("course_code", course.code),
            ("enterprise_name", practice.enterprise_name.clone()),
            ("location", practice.location.clone()),
            ("start_date", format_date(practice.start_date)),
            ("end_date", format_date(practice.end_date)),
            ("supervisor_name", practice.supervisor_name.clone()),
            ("supervisor_email", practice.supervisor_email.clone()),
            ("coordinator_email", coordinator.email.clone()),
            ("coordinator_name", coordinator.name),
        ];

        let pdf_url = self.printer.print(PrintOptions {
            doc_id: practice.id.to_string(),
            template: "document:practice:authotization",
            context: enterprise_auth_pdf_ctx.clone(),
        })?;

        let supervisor_evaluation_url =
            format!("/practices/{}/evaluation/supervisor", practice.id);

        let mut email_context = enterprise_auth_pdf_ctx.clone();

        email_context.push(("practice_authorization_doc_url", pdf_url));
        email_context.push((
            "supervisor_evaluation_url",
            supervisor_evaluation_url.clone(),
        ));

        tokio::try_join!(
            self.mailer.send(MailTo {
                subject: "Información de Práctica Aprobada",
                email: practice.supervisor_email.clone(),
                template: "practice:approval:supervisor",
                context: email_context.clone(),
            }),
            self.mailer.send(MailTo {
                subject: "Práctica Aprobada",
                email: student.email.clone(),
                template: "practice:approval:student",
                context: email_context.clone(),
            }),
            self.mailer.send(MailTo {
                subject: "Práctica Aprobada",
                email: coordinator.email.clone(),
                template: "practice:approval:coordinator",
                context: email_context.clone(),
            }),
        )?;

        practice.is_approved = true;
        self.practices.save(practice).await?;

        Ok("Práctica aprobada exitosamente.".to_string())
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
