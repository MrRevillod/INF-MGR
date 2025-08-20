use std::sync::Arc;
use tokio::sync::{mpsc::Receiver, Mutex};
use uuid::Uuid;

use crate::shared::services::{
    event_queue::{format_date, Event},
    mailer::{MailTo, Mailer},
    printer::{PrintOptions, Printer},
    templates::RawContext,
};

pub struct SubscriberOptions {
    pub rx: Receiver<Event>,
    pub mailer: Mailer,
    pub printer: Printer,
}

pub struct EventSubscriber {
    receiver: Arc<Mutex<Receiver<Event>>>,
    mailer: Arc<Mailer>,
    printer: Arc<Printer>,
}

impl EventSubscriber {
    pub fn new(options: SubscriberOptions) -> Self {
        Self {
            receiver: Arc::new(Mutex::new(options.rx)),
            mailer: Arc::new(options.mailer),
            printer: Arc::new(options.printer),
        }
    }

    pub async fn run_parallel(self) {
        tokio::spawn(async move {
            if let Err(e) = self.subscribe().await {
                eprintln!("Error in event subscriber: {e}");
            }
        });
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
            Event::PracticeApproved((student, practice, course, teacher)) => {
                let enterprise_auth_pdf_ctx: RawContext = vec![
                    ("student_name", student.name),
                    ("course_name", course.name),
                    ("course_code", course.code),
                    ("enterprise_name", practice.enterprise_name),
                    ("location", practice.location),
                    ("start_date", format_date(practice.start_date.to_string())),
                    ("end_date", format_date(practice.end_date.to_string())),
                    ("supervisor_name", practice.supervisor_name),
                    ("supervisor_email", practice.supervisor_email.clone()),
                    ("teacher_name", teacher.name.clone()),
                ];

                let pdf_url = printer
                    .print(PrintOptions {
                        doc_id: practice.id.to_string(),
                        template: "document:practice:authorization",
                        context: enterprise_auth_pdf_ctx.clone(),
                    })
                    .await?;

                let supervisor_evaluation_url =
                    format!("/practices/{}/evaluation/supervisor", practice.id);

                let mut email_context = enterprise_auth_pdf_ctx.clone();

                email_context.push(("practice_authorization_doc_url", pdf_url));
                email_context
                    .push(("supervisor_evaluation_url", supervisor_evaluation_url.clone()));

                tokio::try_join!(
                    mailer.send(MailTo {
                        subject: "Información de Práctica Aprobada",
                        template: "practice:approval:supervisor",
                        email: practice.supervisor_email.clone(),
                        context: email_context.clone(),
                    }),
                    mailer.send(MailTo {
                        subject: "Práctica Aprobada",
                        template: "practice:approval:student",
                        email: student.email,
                        context: email_context.clone(),
                    }),
                    mailer.send(MailTo {
                        subject: "Práctica Aprobada",
                        template: "practice:approval:teacher",
                        email: teacher.email,
                        context: email_context.clone(),
                    }),
                )?;
            }

            Event::PracticeCreated((student, practice, course, enrollment)) => {
                let start_date = format_date(practice.start_date.to_string());
                let end_date = format_date(practice.end_date.to_string());

                let email_context: RawContext = vec![
                    ("student_name", student.name),
                    ("student_email", student.email),
                    ("enterprise_name", practice.enterprise_name),
                    ("supervisor_name", practice.supervisor_name),
                    ("supervisor_email", practice.supervisor_email.clone()),
                    ("course_name", course.name),
                    ("course_code", course.code),
                    ("location", practice.location),
                    ("start_date", start_date.clone()),
                    ("end_date", end_date.clone()),
                    (
                        "approval_link",
                        format!(
                            "/enrollments/{}/practice/{}/approve",
                            enrollment.id,
                            enrollment.practice_id.unwrap_or(Uuid::new_v4())
                        ),
                    ),
                    (
                        "rejection_link",
                        format!(
                            "/enrollments/{}/practice/{}/reject",
                            enrollment.id,
                            enrollment.practice_id.unwrap_or(Uuid::new_v4())
                        ),
                    ),
                ];

                tokio::try_join!(
                    mailer.send(MailTo {
                        email: practice.supervisor_email.clone(),
                        subject: "Solicitud de Inscripción de Práctica",
                        template: "practice:creation:supervisor",
                        context: email_context.clone(),
                    }),
                    mailer.send(MailTo {
                        email: practice.supervisor_email.clone(),
                        subject: "Inscripción a Práctica Aprobada",
                        template: "practice:creation:student",
                        context: email_context,
                    })
                )?;
            }

            Event::UserCreated((name, email, password)) => {
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

            Event::ManyUsersCreated(data) => {
                for (name, email, password) in data {
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

            Event::CourseCreated((course, teacher)) => {
                let context: RawContext = vec![
                    ("course_name", course.name),
                    ("course_code", course.code),
                    ("teacher_name", teacher.name.clone()),
                ];

                let mail_opts = MailTo {
                    subject: "Asignación de Curso",
                    email: teacher.email.clone(),
                    template: "course:creation:teacher",
                    context,
                };

                mailer.send(mail_opts).await?;
            }
        }

        Ok(())
    }
}
