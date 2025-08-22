use std::{env, path::Path, sync::Arc};
use tokio::sync::{mpsc::Receiver, Mutex};
use uuid::Uuid;

use crate::{
    shared::services::{
        event_queue::{format_date, Event},
        mailer::{MailTo, Mailer},
        printer::{PrintOptions, Printer},
        templates::RawContext,
    },
    template_ctx,
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
                tracing::error!("Error in event subscriber: {e}");
            }
        });
    }

    pub async fn subscribe(&self) -> Result<(), Box<dyn std::error::Error>> {
        while let Some(event) = self.receiver.lock().await.recv().await {
            let mailer = self.mailer.clone();
            let printer = self.printer.clone();

            tokio::spawn(async move {
                if let Err(e) = Self::handle(event, mailer, printer).await {
                    tracing::error!("Error processing event: {e}");
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
            Event::PracticeApproved((student, enrollment, practice, course, teacher)) => {
                let mut template_ctx = template_ctx! {
                    "student_rut" => student.rut,
                    "student_name" => student.name,
                    "course_name" => course.name,
                    "course_code" => course.code,
                    "enterprise_name" => practice.enterprise_name,
                    "location" => practice.location,
                    "start_date" => format_date(practice.start_date.to_string()),
                    "end_date" => format_date(practice.end_date.to_string()),
                    "supervisor_name" => practice.supervisor_name,
                    "supervisor_email" => practice.supervisor_email.clone(),
                    "teacher_name" => teacher.name.clone(),
                };

                let practice_static_dir = format!("practices/{}", practice.id);
                let practice_auth_doc = format!("/static/{practice_static_dir}/authorization.pdf");

                let print_opts = PrintOptions {
                    static_path: format!("{practice_static_dir}/authorization.pdf"),
                    template: "document:practice:authorization",
                    context: template_ctx.clone(),
                };

                printer.print(print_opts).await?;

                template_ctx.push(("practice_auth_doc_url", practice_auth_doc));
                template_ctx.push((
                    "practice_auth_form_url",
                    format!("/enrollments/{}/practice/{}/authorize", enrollment.id, practice.id),
                ));

                template_ctx.push((
                    "practice_evaluation_form_url",
                    format!("/enrollments/{}/practice/{}/evaluate", enrollment.id, practice.id),
                ));

                let (_, _, _, _) = tokio::join!(
                    mailer.send(MailTo {
                        subject: "Información de Práctica Aprobada",
                        template: "practice:approval:supervisor",
                        email: practice.supervisor_email.clone(),
                        context: template_ctx.clone(),
                    }),
                    mailer.send(MailTo {
                        subject: "Práctica Aprobada",
                        template: "practice:approval:student",
                        email: student.email,
                        context: template_ctx.clone(),
                    }),
                    mailer.send(MailTo {
                        subject: "Práctica Aprobada",
                        template: "practice:approval:teacher",
                        email: teacher.email,
                        context: template_ctx.clone()
                    }),
                    mailer.send(MailTo {
                        subject: "Práctica Aprobada",
                        template: "practice:approval:secretary",
                        context: template_ctx,
                        email: mailer.context().config().secretary_email.clone(),
                    })
                );
            }

            Event::PracticeDeclined((student, practice, course, teacher)) => {
                let email_context: RawContext = vec![
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

                tokio::try_join!(
                    mailer.send(MailTo {
                        subject: "Inscripción a Práctica Rechazada",
                        template: "practice:decline:supervisor",
                        email: practice.supervisor_email.clone(),
                        context: email_context.clone(),
                    }),
                    mailer.send(MailTo {
                        subject: "Inscripción a Práctica Rechazada",
                        template: "practice:decline:student",
                        email: student.email,
                        context: email_context.clone(),
                    }),
                    mailer.send(MailTo {
                        subject: "Inscripción a Práctica Rechazada",
                        template: "practice:decline:teacher",
                        email: teacher.email,
                        context: email_context,
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

            Event::PracticeAuthorized((practice, pdf)) => {
                let practice_static_dir = format!("practices/{}/authorization.pdf", practice.id);
                let documents_dir = env::var("DOCUMENTS_DIR").unwrap_or(".".to_string());
                let out_path_str = format!("{documents_dir}/{practice_static_dir}");
                let out_path = Path::new(&out_path_str);

                if let Some(parent) = out_path.parent() {
                    if !parent.exists() {
                        std::fs::create_dir_all(parent)?;
                    }
                }

                tokio::fs::write(out_path, pdf).await?;
            }
        }

        Ok(())
    }
}
