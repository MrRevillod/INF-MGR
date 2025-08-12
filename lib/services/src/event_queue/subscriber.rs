use std::sync::Arc;
use tokio::sync::{Mutex, mpsc::Receiver};

use crate::{event_queue::Event, mailer::Mailer, printer::Printer};

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
        _: Arc<Mailer>,
        _: Arc<Printer>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match event {
            Event::PracticeApproved(data) => {
                println!("Practice approved: {data:?}");
            }

            Event::PracticeCreated(data) => {
                println!("Practice created: {data:?}");
            }

            Event::UserCreated(data) => {
                println!("User created: {data:?}");

                // let context: RawContext = vec![
                //     ("name", data.get("name").unwrap_or(&Value::Null).to_string()),
                //     (
                //         "email",
                //         data.get("email").unwrap_or(&Value::Null).to_string(),
                //     ),
                //     (
                //         "password",
                //         data.get("password").unwrap_or(&Value::Null).to_string(),
                //     ),
                // ];

                // let mail_opts = MailTo {
                //     subject: "Bienvenido (a) a la plataforma",
                //     email: input.email.clone(),
                //     template: "system:welcome",
                //     context,
                // };

                // mailer.send(mail_opts).await?;
            } // ==================

              // let email_context: RawContext = vec![
              //     ("student_name", student.name.clone()),
              //     ("student_email", student.email.clone()),
              //     ("enterprise_name", practice.enterprise_name.clone()),
              //     ("supervisor_name", practice.supervisor_name.clone()),
              //     ("supervisor_email", practice.supervisor_email.clone()),
              //     ("course_name", course.name.clone()),
              //     ("course_code", course.code.clone()),
              //     ("location", practice.location.clone()),
              //     ("start_date", start_date.unwrap_or_default()),
              //     ("end_date", end_date.unwrap_or_default()),
              //     (
              //         "approval_link",
              //         format!(
              //             "/enrollments/{}/practice/{}/approve",
              //             enrollment.id, practice.id
              //         ),
              //     ),
              //     (
              //         "rejection_link",
              //         format!(
              //             "/enrollments/{}/practice/{}/reject",
              //             enrollment.id, practice.id
              //         ),
              //     ),
              // ];

              // tokio::try_join!(
              //     self.mailer.send(MailTo {
              //         email: practice.supervisor_email.clone(),
              //         subject: "Solicitud de Inscripción de Práctica",
              //         template: "practice:creation:supervisor",
              //         context: email_context.clone(),
              //     }),
              //     self.mailer.send(MailTo {
              //         email: student.email.clone(),
              //         subject: "Inscripción a Práctica Aprobada",
              //         template: "practice:creation:student",
              //         context: email_context,
              //     })
              // )?;

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
