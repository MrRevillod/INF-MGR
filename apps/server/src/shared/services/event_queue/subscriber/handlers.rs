use std::{env, path::Path};
use uuid::Uuid;

use crate::shared::{AppResult, services::*};

#[derive(Clone)]
pub struct SubscriberHandler {
    pub printer: Printer,
    pub mailer: Mailer,
}

impl SubscriberHandler {
    pub async fn practice_approved(
        &self,
        event: PracticeApprovedEvent,
    ) -> AppResult<()> {
        let (student, enrollment, practice, course, teacher) = event;
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
        let practice_auth_doc =
            format!("/static/{practice_static_dir}/authorization.pdf");

        let print_opts = PrintOptions {
            static_path: format!("{practice_static_dir}/authorization.pdf"),
            template: "document:practice:authorization",
            context: template_ctx.clone(),
        };

        self.printer.print(print_opts).await?;

        template_ctx.push(("practice_auth_doc_url", practice_auth_doc));
        template_ctx.push((
            "practice_auth_form_url",
            format!(
                "/enrollments/{}/practice/{}/authorize",
                enrollment.id, practice.id
            ),
        ));

        template_ctx.push((
            "practice_evaluation_form_url",
            format!(
                "/enrollments/{}/practice/{}/evaluate",
                enrollment.id, practice.id
            ),
        ));

        self.mailer
            .send_many(vec![
                MailTo {
                    subject: "Información de Práctica Aprobada",
                    template: "practice:approval:supervisor",
                    email: practice.supervisor_email.clone(),
                    context: template_ctx.clone(),
                },
                MailTo {
                    subject: "Práctica Aprobada",
                    template: "practice:approval:student",
                    email: student.email,
                    context: template_ctx.clone(),
                },
                MailTo {
                    subject: "Práctica Aprobada",
                    template: "practice:approval:teacher",
                    email: teacher.email.clone(),
                    context: template_ctx.clone(),
                },
                MailTo {
                    subject: "Práctica Aprobada",
                    template: "practice:approval:secretary",
                    context: template_ctx,
                    email: self.mailer.context().config().secretary_email.clone(),
                },
            ])
            .await?;

        Ok(())
    }

    pub async fn practice_declined(&self, event: PracticeDeclinedEvent) {
        let (student, _, practice, course, teacher) = event;
        let email_context = template_ctx! {
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

        let (_, _, _) = tokio::join!(
            self.mailer.send(MailTo {
                subject: "Inscripción a Práctica Rechazada",
                template: "practice:decline:supervisor",
                email: practice.supervisor_email.clone(),
                context: email_context.clone(),
            }),
            self.mailer.send(MailTo {
                subject: "Inscripción a Práctica Rechazada",
                template: "practice:decline:student",
                email: student.email,
                context: email_context.clone(),
            }),
            self.mailer.send(MailTo {
                subject: "Inscripción a Práctica Rechazada",
                template: "practice:decline:teacher",
                email: teacher.email,
                context: email_context,
            }),
        );
    }

    pub async fn practice_created(&self, event: PracticeCreatedEvent) {
        let (student, practice, course, enrollment) = event;

        let start_date = format_date(practice.start_date.to_string());
        let end_date = format_date(practice.end_date.to_string());

        let approval_link = format!(
            "/enrollments/{}/practice/{}/approve",
            enrollment.id,
            enrollment.practice_id.unwrap_or(Uuid::new_v4())
        );

        let rejection_link = format!(
            "/enrollments/{}/practice/{}/reject",
            enrollment.id,
            enrollment.practice_id.unwrap_or(Uuid::new_v4())
        );

        let email_context = template_ctx! {
            "student_name" => student.name,
            "student_email" => student.email,
            "enterprise_name" => practice.enterprise_name,
            "supervisor_name" => practice.supervisor_name,
            "supervisor_email" => practice.supervisor_email.clone(),
            "course_name" => course.name,
            "course_code" => course.code,
            "location" => practice.location,
            "start_date" => start_date.clone(),
            "end_date" => end_date.clone(),
            "approval_link" => approval_link,
            "rejection_link" => rejection_link
        };

        let (_, _) = tokio::join!(
            self.mailer.send(MailTo {
                email: practice.supervisor_email.clone(),
                subject: "Solicitud de Inscripción de Práctica",
                template: "practice:creation:supervisor",
                context: email_context.clone(),
            }),
            self.mailer.send(MailTo {
                email: practice.supervisor_email.clone(),
                subject: "Inscripción a Práctica Realizada",
                template: "practice:creation:student",
                context: email_context,
            })
        );
    }

    pub async fn user_created(
        &self,
        (name, email): UserCreatedEvent,
    ) -> AppResult<()> {
        let context: RawContext = template_ctx! {
            "name" => name,
            "email" => email.clone(),
        };

        let mail_opts = MailTo {
            subject: "Bienvenido (a) a la plataforma",
            email,
            template: "system:welcome",
            context,
        };

        self.mailer.send(mail_opts).await?;

        Ok(())
    }

    pub async fn many_users_created(
        &self,
        data: ManyUsersCreatedEvent,
    ) -> AppResult<()> {
        for user in data {
            self.user_created(user).await?;
        }

        Ok(())
    }

    pub async fn course_created(
        &self,
        (course, teacher): CourseCreatedEvent,
    ) -> AppResult<()> {
        let context = template_ctx! {
            "course_name" => course.name,
            "course_code" => course.code,
            "teacher_name" => teacher.name.clone(),
        };

        let mail_opts = MailTo {
            subject: "Asignación de Curso",
            email: teacher.email.clone(),
            template: "course:creation:teacher",
            context,
        };

        self.mailer.send(mail_opts).await?;

        Ok(())
    }

    pub async fn practice_authorized(
        &self,
        (practice, doc_bytes): PracticeAuthorizedEvent,
    ) -> AppResult<()> {
        let practice_static_dir =
            format!("practices/{}/authorization.pdf", practice.id);

        let documents_dir = env::var("DOCUMENTS_DIR").unwrap_or(".".to_string());
        let out_path_str = format!("{documents_dir}/{practice_static_dir}");
        let out_path = Path::new(&out_path_str);

        if let Some(parent) = out_path.parent()
            && !parent.exists()
        {
            std::fs::create_dir_all(parent).map_err(|source| {
                ServiceError::Printer {
                    source: source.into(),
                }
            })?;
        }

        tokio::fs::write(out_path, doc_bytes)
            .await
            .map_err(|source| ServiceError::Printer {
                source: source.into(),
            })?;

        Ok(())
    }

    pub async fn practice_evaluated(&self, event: PracticeEvaluatedEvent) {
        let (student, practice, course, teacher, score) = event;
        let email_context = template_ctx! {
            "student_name" => student.name,
            "student_email" => student.email,
            "enterprise_name" => practice.enterprise_name,
            "supervisor_name" => practice.supervisor_name,
            "supervisor_email" => practice.supervisor_email.clone(),
            "course_name" => course.name,
            "course_code" => course.code,
            "location" => practice.location,
            "start_date" => format_date(practice.start_date.to_string()),
            "end_date" => format_date(practice.end_date.to_string()),
            "teacher_name" => teacher.name.clone(),
            "evaluation_score" => score.to_string(),
            "evaluation_comments" => "Evaluación completada por el supervisor".to_string(),
        };

        let (_, _) = tokio::join!(
            self.mailer.send(MailTo {
                subject: "Práctica Evaluada",
                template: "practice:evaluation:teacher",
                email: teacher.email.clone(),
                context: email_context.clone(),
            }),
            self.mailer.send(MailTo {
                subject: "Evaluación de Práctica Completada",
                template: "practice:evaluation:supervisor",
                email: practice.supervisor_email.clone(),
                context: email_context,
            }),
        );
    }
}
