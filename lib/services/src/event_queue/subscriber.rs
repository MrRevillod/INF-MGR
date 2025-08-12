use serde_json::Value;
use std::sync::Arc;
use tokio::sync::{Mutex, mpsc::Receiver};

use crate::{
    event_queue::{Event, format_date, get_json},
    mailer::{MailTo, Mailer},
    printer::Printer,
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
        _: Arc<Printer>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            Event::PracticeApproved(data) => {
                println!("Practice approved: {data:?}");
            }

            Event::PracticeCreated(data) => {
                let student = get_json::<Value>(&data, "student")?;
                let practice = get_json::<Value>(&data, "practice")?;
                let course = get_json::<Value>(&data, "course")?;
                let enrollment = get_json::<Value>(&data, "enrollment")?;
                let start_date = get_json::<String>(&practice, "start_date")?;
                let end_date = get_json::<String>(&practice, "end_date")?;
                let email_student = get_json::<String>(&student, "email")?;
                let email_supervisor =
                    get_json::<String>(&practice, "supervisor_email")?;

                let email_context: RawContext = vec![
                    (
                        "student_name",
                        get_json::<String>(&student, "student_name")?,
                    ),
                    ("student_email", email_student.clone()),
                    (
                        "enterprise_name",
                        get_json::<String>(&practice, "enterprise_name")?,
                    ),
                    (
                        "supervisor_name",
                        get_json::<String>(&practice, "supervisor_name")?,
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
                            get_json::<String>(&enrollment, "practice_id")?
                        ),
                    ),
                    (
                        "rejection_link",
                        format!(
                            "/enrollments/{}/practice/{}/reject",
                            get_json::<String>(&enrollment, "id")?,
                            get_json::<String>(&enrollment, "practice_id")?
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
            } // ==================

              // ==================

              // let enterprise_auth_pdf_ctx: RawContext = vec![
              //     ("student_name", student.name),
              //     ("course_name", course.name),
              //     ("course_code", course.code),
              //     ("enterprise_name", practice.enterprise_name.clone()),
              //     ("location", practice.location.clone()),
              //     ("start_date", format_date(practice.start_date)),
              //     ("end_date", format_date(practice.end_date)),
              //     ("supervisor_name", practice.supervisor_name.clone()),
              //     ("supervisor_email", practice.supervisor_email.clone()),
              //     ("coordinator_email", coordinator.email.clone()),
              //     ("coordinator_name", coordinator.name),
              // ];

              // let pdf_url = self
              //     .printer
              //     .print(PrintOptions {
              //         doc_id: practice.id.to_string(),
              //         template: "document:practice:authorization",
              //         context: enterprise_auth_pdf_ctx.clone(),
              //     })
              //     .await?;

              // let supervisor_evaluation_url =
              //     format!("/practices/{}/evaluation/supervisor", practice.id);

              // let mut email_context = enterprise_auth_pdf_ctx.clone();

              // email_context.push(("practice_authorization_doc_url", pdf_url));
              // email_context.push((
              //     "supervisor_evaluation_url",
              //     supervisor_evaluation_url.clone(),
              // ));

              // tokio::try_join!(
              //     self.mailer.send(MailTo {
              //         subject: "Información de Práctica Aprobada",
              //         email: practice.supervisor_email.clone(),
              //         template: "practice:approval:supervisor",
              //         context: email_context.clone(),
              //     }),
              //     self.mailer.send(MailTo {
              //         subject: "Práctica Aprobada",
              //         email: student.email.clone(),
              //         template: "practice:approval:student",
              //         context: email_context.clone(),
              //     }),
              //     self.mailer.send(MailTo {
              //         subject: "Práctica Aprobada",
              //         email: coordinator.email.clone(),
              //         template: "practice:approval:coordinator",
              //         context: email_context.clone(),
              //     }),
              // )?;
        }

        Ok(())
    }
}
