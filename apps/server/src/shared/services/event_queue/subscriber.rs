use serde_json::Value;
use std::sync::Arc;
use tokio::sync::{mpsc::Receiver, Mutex};

use crate::shared::services::{
    event_queue::{format_date, get_json, Event},
    mailer::{MailTo, Mailer},
    printer::{PrintOptions, Printer},
    templates::RawContext,
};

pub struct SubscriberServices {
    pub mailer: Mailer,
    pub printer: Printer,
}

pub struct EventSubscriber {
    receiver: Arc<Mutex<Receiver<Event>>>,
    mailer: Arc<Mailer>,
    printer: Arc<Printer>,
}

impl EventSubscriber {
    pub fn new(rcv: Receiver<Event>, services: SubscriberServices) -> Self {
        Self {
            receiver: Arc::new(Mutex::new(rcv)),
            mailer: Arc::new(services.mailer),
            printer: Arc::new(services.printer),
        }
    }

    pub async fn subscribe(&self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(event) = self.receiver.lock().await.recv().await {
            let mailer = self.mailer.clone();
            let printer = self.printer.clone();

            tokio::spawn(async move {
                if let Err(e) = Self::handle(event, mailer, printer).await {
                    eprintln!("Error processing event: {e}");
                }
            });
        }

        Ok(())
    }

    async fn handle(
        event: Event,
        mailer: Arc<Mailer>,
        printer: Arc<Printer>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            Event::PracticeApproved(data) => {
                let student = get_json::<Value>(&data, "student")?;
                let practice = get_json::<Value>(&data, "practice")?;
                let course = get_json::<Value>(&data, "course")?;
                let teacher = get_json::<Value>(&data, "teacher")?;
                let email_student = get_json::<String>(&student, "email")?;
                let email_teacher = get_json::<String>(&teacher, "email")?;
                let email_supervisor =
                    get_json::<String>(&practice, "supervisorEmail")?;

                let enterprise_auth_pdf_ctx: RawContext = vec![
                    ("student_name", get_json::<String>(&student, "name")?),
                    ("course_name", get_json::<String>(&course, "name")?),
                    ("course_code", get_json::<String>(&course, "code")?),
                    (
                        "enterprise_name",
                        get_json::<String>(&practice, "enterpriseName")?,
                    ),
                    ("location", get_json::<String>(&practice, "location")?),
                    (
                        "start_date",
                        format_date(get_json::<String>(&practice, "startDate")?),
                    ),
                    (
                        "end_date",
                        format_date(get_json::<String>(&practice, "endDate")?),
                    ),
                    (
                        "supervisor_name",
                        get_json::<String>(&practice, "supervisorName")?,
                    ),
                    (
                        "supervisor_email",
                        get_json::<String>(&practice, "supervisorEmail")?,
                    ),
                    ("teacher_name", get_json::<String>(&teacher, "name")?),
                ];

                let pdf_url = printer
                    .print(PrintOptions {
                        doc_id: get_json::<String>(&practice, "id")?,
                        template: "document:practice:authorization",
                        context: enterprise_auth_pdf_ctx.clone(),
                    })
                    .await?;
                let supervisor_evaluation_url = format!(
                    "/practices/{}/evaluation/supervisor",
                    get_json::<String>(&practice, "id")?
                );

                let mut email_context = enterprise_auth_pdf_ctx.clone();

                email_context.push(("practice_authorization_doc_url", pdf_url));
                email_context.push((
                    "supervisor_evaluation_url",
                    supervisor_evaluation_url.clone(),
                ));

                tokio::try_join!(
                    mailer.send(MailTo {
                        subject: "Información de Práctica Aprobada",
                        email: email_supervisor,
                        template: "practice:approval:supervisor",
                        context: email_context.clone(),
                    }),
                    mailer.send(MailTo {
                        subject: "Práctica Aprobada",
                        email: email_student,
                        template: "practice:approval:student",
                        context: email_context.clone(),
                    }),
                    mailer.send(MailTo {
                        subject: "Práctica Aprobada",
                        email: email_teacher,
                        template: "practice:approval:teacher",
                        context: email_context.clone(),
                    }),
                )?;

                println!("Practice approved: {student:?}, {practice:?}, {course:?}");
            }

            Event::PracticeCreated(data) => {
                let student = get_json::<Value>(&data, "student")?;
                let practice = get_json::<Value>(&data, "practice")?;
                let course = get_json::<Value>(&data, "course")?;
                let enrollment = get_json::<Value>(&data, "enrollment")?;
                let start_date = get_json::<String>(&practice, "startDate")?;
                let end_date = get_json::<String>(&practice, "endDate")?;
                let email_student = get_json::<String>(&student, "email")?;
                let email_supervisor =
                    get_json::<String>(&practice, "supervisorEmail")?;

                let email_context: RawContext = vec![
                    ("student_name", get_json::<String>(&student, "name")?),
                    ("student_email", email_student.clone()),
                    (
                        "enterprise_name",
                        get_json::<String>(&practice, "enterpriseName")?,
                    ),
                    (
                        "supervisor_name",
                        get_json::<String>(&practice, "supervisorName")?,
                    ),
                    ("supervisor_email", email_supervisor.clone()),
                    ("course_name", get_json::<String>(&course, "name")?),
                    ("course_code", get_json::<String>(&course, "code")?),
                    ("location", get_json::<String>(&practice, "location")?),
                    ("start_date", format_date(start_date)),
                    ("end_date", format_date(end_date)),
                    (
                        "approval_link",
                        format!(
                            "/enrollments/{}/practice/{}/approve",
                            get_json::<String>(&enrollment, "id")?,
                            get_json::<String>(&enrollment, "practiceId")?
                        ),
                    ),
                    (
                        "rejection_link",
                        format!(
                            "/enrollments/{}/practice/{}/reject",
                            get_json::<String>(&enrollment, "id")?,
                            get_json::<String>(&enrollment, "practiceId")?
                        ),
                    ),
                ];

                tokio::try_join!(
                    mailer.send(MailTo {
                        email: email_supervisor,
                        subject: "Solicitud de Inscripción de Práctica",
                        template: "practice:creation:supervisor",
                        context: email_context.clone(),
                    }),
                    mailer.send(MailTo {
                        email: email_student,
                        subject: "Inscripción a Práctica Aprobada",
                        template: "practice:creation:student",
                        context: email_context,
                    })
                )?;
            }

            Event::UserCreated(data) => {
                let name = get_json::<String>(&data, "name")?;
                let email = get_json::<String>(&data, "email")?;
                let password = get_json::<String>(&data, "password")?;

                let context: RawContext = vec![
                    ("name", name),
                    ("email", email.clone()),
                    ("password", password),
                ];

                let mail_opts = MailTo {
                    subject: "Bienvenido (a) a la plataforma",
                    email,
                    template: "system:welcome",
                    context,
                };

                mailer.send(mail_opts).await?;
            }
        }

        Ok(())
    }
}
