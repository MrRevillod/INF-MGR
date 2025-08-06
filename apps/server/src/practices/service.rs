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
    mailer: Arc<dyn Mailer>,

    #[shaku(inject)]
    printer: Arc<dyn Printer>,
}

#[async_trait]
pub trait PracticeService: Interface {
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Practice>, AppError>;

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

    async fn approve(&self, id: &Uuid) -> AppResult<String>;
}

#[async_trait]
impl PracticeService for PracticeServiceImpl {
    async fn get_by_id(&self, id: &Uuid) -> Result<Option<Practice>, AppError> {
        self.practices.find_by_id(id).await
    }

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
                format!("/practices/{}/approve", practice.id),
            ),
            (
                "rejection_link",
                format!("/practices/{}/reject", practice.id),
            ),
        ];

        let (_, _) = tokio::try_join!(
            self.mailer.send(MailTo {
                email: practice.supervisor_email.clone(),
                subject: "Solicitud de Inscripción de Práctica",
                template: "practices:creation:supervisor",
                context: email_context.clone(),
            }),
            self.mailer.send(MailTo {
                email: student.email.clone(),
                subject: "Inscripción a Práctica Aprobada",
                template: "practices:creation:student",
                context: email_context,
            }),
        )?;

        let practice = self.practices.save(practice).await?;

        enrollment.practice_id = Some(practice.id);
        self.enrollments.save(enrollment).await?;

        Ok(practice)
    }

    async fn approve(&self, id: &Uuid) -> AppResult<String> {
        let mut practice = self.practices.find_by_id(id).await?.ok_or(
            AppError::ResourceNotFound {
                id: id.to_string(),
                kind: "Practice",
            },
        )?;

        let (enrollment, student, _) =
            self.enrollments.get_by_id(&practice.enrollment_id).await?;

        let course = self.courses.get_by_id(&enrollment.course_id).await?;

        let template_ctx = RawContext::from(vec![
            ("enterprise_name", practice.enterprise_name.clone()),
            ("supervisor_name", practice.supervisor_name.clone()),
            ("supervisor_email", practice.supervisor_email.clone()),
            ("course_name", course.name.clone()),
            ("student_name", student.name.clone()),
            ("student_email", student.email.clone()),
        ]);

        let print_opts = PrintOptions {
            doc_id: practice.id.to_string(),
            template: "document:practice:authotization",
            context: template_ctx,
        };

        let pdf_data = self.printer.print(print_opts)?;

        println!("Generated PDF Data: {}", pdf_data);

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
