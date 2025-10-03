use crate::{
    imports::ImportedStudent,
    shared::{AppResult, services::*},
};
use uuid::Uuid;

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
            "supervisor_name" => practice.supervisor_name,
            "supervisor_email" => practice.supervisor_email.clone(),
            "supervisor_phone" => practice.supervisor_phone,
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
                    subject: "Información de Práctica Aprobada".into(),
                    template: "practice:approval:supervisor",
                    email: practice.supervisor_email.clone(),
                    context: template_ctx.clone(),
                },
                MailTo {
                    subject: "Práctica Aprobada".into(),
                    template: "practice:approval:student",
                    email: student.email,
                    context: template_ctx.clone(),
                },
                MailTo {
                    subject: "Práctica Aprobada".into(),
                    template: "practice:approval:teacher",
                    email: teacher.email.clone(),
                    context: template_ctx.clone(),
                },
                MailTo {
                    subject: "Práctica Aprobada".into(),
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
            "student_name" => student.name.clone(),
            "course_name" => course.name.clone(),
            "course_code" => course.code.clone(),
            "enterprise_name" => practice.enterprise_name.clone(),
            "location" => practice.location.clone(),
            "supervisor_name" => practice.supervisor_name.clone(),
            "supervisor_email" => practice.supervisor_email.clone(),
            "teacher_email" => teacher.email.clone(),
        };

        let (_, _, _, _) = tokio::join!(
            self.mailer.send(MailTo {
                subject: "Inscripción a Práctica Rechazada".into(),
                template: "practice:decline:supervisor",
                email: practice.supervisor_email.clone(),
                context: email_context.clone(),
            }),
            self.mailer.send(MailTo {
                subject: "Inscripción a Práctica Rechazada".into(),
                template: "practice:decline:student",
                email: student.email,
                context: email_context.clone(),
            }),
            self.mailer.send(MailTo {
                subject: "Inscripción a Práctica Rechazada".into(),
                template: "practice:decline:teacher",
                email: teacher.email.clone(),
                context: email_context.clone(),
            }),
            self.mailer.send(MailTo {
                subject: "Inscripción a Práctica Rechazada".into(),
                template: "practice:decline:secretary",
                email: self.mailer.context().config().secretary_email.clone(),
                context: email_context,
            })
        );
    }

    pub async fn practice_created(&self, event: PracticeCreatedEvent) {
        let (student, practice, course, enrollment) = event;

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
            "student_name" => student.name.clone(),
            "enterprise_name" => practice.enterprise_name.clone(),
            "supervisor_name" => practice.supervisor_name.clone(),
            "course_name" => course.name.clone(),
            "course_code" => course.code.clone(),
            "approval_link" => approval_link,
            "rejection_link" => rejection_link
        };

        let (_, _, _) = tokio::join!(
            self.mailer.send(MailTo {
                email: practice.supervisor_email.clone(),
                subject: "Solicitud de Inscripción de Práctica".into(),
                template: "practice:creation:supervisor",
                context: email_context.clone(),
            }),
            self.mailer.send(MailTo {
                email: student.email.clone(),
                subject: "Inscripción a Práctica Realizada".into(),
                template: "practice:creation:student",
                context: email_context.clone(),
            }),
            self.mailer.send(MailTo {
                email: self.mailer.context().config().secretary_email.clone(),
                subject: "Nueva Inscripción de Práctica".into(),
                template: "practice:creation:secretary",
                context: email_context.clone(),
            })
        );
    }

    pub async fn user_created(
        &self,
        (name, email): UserCreatedEvent,
    ) -> AppResult<()> {
        let context: RawContext = template_ctx! {
            "name" => name,
        };

        let mail_opts = MailTo {
            subject: "Bienvenido (a) a la plataforma".into(),
            email,
            template: "system:welcome",
            context,
        };

        self.mailer.send(mail_opts).await?;

        Ok(())
    }

    pub async fn imported_students(
        &self,
        data: Vec<ImportedStudent>,
    ) -> AppResult<()> {
        for user in data {
            self.user_created((user.name, user.email)).await?;
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
        };

        let mail_opts = MailTo {
            subject: "Asignación de Curso".into(),
            email: teacher.email.clone(),
            template: "course:creation:teacher",
            context,
        };

        self.mailer.send(mail_opts).await?;

        Ok(())
    }

    pub async fn practice_authorized(
        &self,
        event: PracticeAuthorizedEvent,
    ) -> AppResult<()> {
        let (student, course, teacher, practice, doc_bytes) = event;
        let practice_static_dir =
            format!("practices/{}/authorization.pdf", practice.id);

        let context = template_ctx! {
            "student_name" => student.name.clone(),
            "student_rut" => student.rut.clone(),
            "enterprise_name" => practice.enterprise_name.clone(),
            "course_code" => course.code.clone(),
        };

        self.printer.archive(practice_static_dir, doc_bytes).await?;

        let (_, _) = tokio::join!(
            self.mailer.send(MailTo {
                subject: "Práctica Autorizada".into(),
                template: "practice:authorization:secretary",
                email: self.mailer.context().config().secretary_email.clone(),
                context: context.clone(),
            }),
            self.mailer.send(MailTo {
                subject: "Práctica Autorizada".into(),
                template: "practice:authorization:teacher",
                email: teacher.email.clone(),
                context
            }),
        );

        Ok(())
    }

    pub async fn practice_evaluated(&self, event: PracticeEvaluatedEvent) {
        let (student, practice, course, teacher, _score) = event;
        let email_context = template_ctx! {
            "student_name" => student.name.clone(),
            "enterprise_name" => practice.enterprise_name.clone(),
            "supervisor_name" => practice.supervisor_name.clone(),
            "course_name" => course.name.clone(),
            "course_code" => course.code.clone(),
        };

        let (_, _) = tokio::join!(
            self.mailer.send(MailTo {
                subject: "Práctica Evaluada".into(),
                template: "practice:evaluation:teacher",
                email: teacher.email.clone(),
                context: email_context.clone(),
            }),
            self.mailer.send(MailTo {
                subject: "Evaluación de Práctica Completada".into(),
                template: "practice:evaluation:supervisor",
                email: practice.supervisor_email.clone(),
                context: email_context,
            }),
        );
    }

    pub async fn final_report_uploaded(
        &self,
        event: FinalReportUploadedEvent,
    ) -> AppResult<()> {
        let (enrollment, course, teacher, student, doc_bytes) = event;

        let report_static_path = format!(
            "practices/{}/report.pdf",
            enrollment.practice_id.unwrap_or_default()
        );

        self.printer.archive(report_static_path, doc_bytes).await?;

        let subject = format!("{} - Informe final subido", course.code);

        let ctx = template_ctx! {
            "student_name" => student.name.clone(),
            "course_name" => course.name.clone(),
            "course_code" => course.code.clone(),
        };

        let (_, _) = tokio::join!(
            self.mailer.send(MailTo {
                subject: subject.clone(),
                template: "report-upload:teacher",
                email: teacher.email.clone(),
                context: ctx.clone(),
            }),
            self.mailer.send(MailTo {
                subject,
                template: "report-upload:student",
                email: student.email.clone(),
                context: ctx,
            }),
        );

        Ok(())
    }

    pub async fn meeting_request_created(
        &self,
        event: MeetingRequestCreatedEvent,
    ) -> AppResult<()> {
        let (teacher, course, students) = event;

        let student_names: Vec<String> =
            students.iter().map(|s| s.name.clone()).collect();

        let formatted_student_names = if student_names.len() > 1 {
            let last = student_names.last().cloned().unwrap_or_default();
            let others = &student_names[..student_names.len() - 1];
            format!("{} y {}", others.join(", "), last)
        } else {
            student_names.first().cloned().unwrap_or_default()
        };

        let context = template_ctx! {
            "teacher_name" => teacher.name.clone(),
            "student_names" => formatted_student_names,
            "course_code" => course.code.clone(),
            "course_name" => course.name.clone(),
        };

        let mail_opts = MailTo {
            subject: format!("{} - Nueva solicitud de reunión", course.code),
            email: teacher.email.clone(),
            template: "meeting-req:creation:teacher",
            context,
        };

        self.mailer.send(mail_opts).await?;

        Ok(())
    }
}
