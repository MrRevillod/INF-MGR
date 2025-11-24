use super::events::*;
use crate::{imports::ImportedStudent, send_emails, template_ctx};

use services::{
    ServiceResult, embeddings::EmbeddingService, mailer::*,
    plagiarism_new::PlagiarismDetectionService, printer::*, types::*,
};

#[derive(Clone)]
pub struct SubscriberHandler {
    pub printer: Printer,
    pub mailer: Mailer,
    pub embedding_service: EmbeddingService,
    pub plagiarism_service: PlagiarismDetectionService,
}

impl SubscriberHandler {
    pub async fn practice_approved(
        &self,
        event: PracticeApprovedEvent,
    ) -> ServiceResult<()> {
        let (student, enrollment, practice, course, teacher) = event;

        let student_register = student
            .register
            .clone()
            .unwrap_or_else(|| "Sin registro".to_string());

        let mut template_ctx = template_ctx! {
            "student_rut" => student.rut,
            "student_name" => student.name,
            "student_register" => student_register,
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

        send_emails! {
            &self.mailer,
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
            }
        };

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

        send_emails!(
            &self.mailer,
            MailTo {
                subject: "Inscripción a Práctica Rechazada".into(),
                template: "practice:decline:supervisor",
                email: practice.supervisor_email.clone(),
                context: email_context.clone(),
            },
            MailTo {
                subject: "Inscripción a Práctica Rechazada".into(),
                template: "practice:decline:student",
                email: student.email,
                context: email_context.clone(),
            },
            MailTo {
                subject: "Inscripción a Práctica Rechazada".into(),
                template: "practice:decline:teacher",
                email: teacher.email.clone(),
                context: email_context.clone(),
            },
            MailTo {
                subject: "Inscripción a Práctica Rechazada".into(),
                template: "practice:decline:secretary",
                email: self.mailer.context().config().secretary_email.clone(),
                context: email_context,
            }
        );
    }

    pub async fn practice_created(&self, event: PracticeCreatedEvent) {
        let (student, practice, course, enrollment) = event;

        let approval_link = format!(
            "/enrollments/{}/practice/{}/approve",
            enrollment.id,
            enrollment.practice_id.unwrap_or_else(Uuid::new_v4)
        );

        let rejection_link = format!(
            "/enrollments/{}/practice/{}/reject",
            enrollment.id,
            enrollment.practice_id.unwrap_or_else(Uuid::new_v4)
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

        send_emails! {
            &self.mailer,
            MailTo {
                email: practice.supervisor_email.clone(),
                subject: "Solicitud de Inscripción de Práctica".into(),
                template: "practice:creation:supervisor",
                context: email_context.clone(),
            },
            MailTo {
                email: student.email.clone(),
                subject: "Inscripción a Práctica Realizada".into(),
                template: "practice:creation:student",
                context: email_context.clone(),
            },
            MailTo {
                email: self.mailer.context().config().secretary_email.clone(),
                subject: "Nueva Inscripción de Práctica".into(),
                template: "practice:creation:secretary",
                context: email_context.clone(),
            }
        };
    }

    pub async fn user_created(&self, event: UserCreatedEvent) -> ServiceResult<()> {
        let (name, email) = event;

        let context = template_ctx! {
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
    ) -> ServiceResult<()> {
        for user in data {
            self.user_created((user.name, user.email)).await?;
        }

        Ok(())
    }

    pub async fn course_created(
        &self,
        (course, teacher): CourseCreatedEvent,
    ) -> ServiceResult<()> {
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
    ) -> ServiceResult<()> {
        let (student, course, teacher, practice, doc_bytes) = event;
        let practice_static_dir = format!("practices/{}/authorization.pdf", practice.id);

        let context = template_ctx! {
            "student_name" => student.name.clone(),
            "student_rut" => student.rut.clone(),
            "enterprise_name" => practice.enterprise_name.clone(),
            "course_code" => course.code.clone(),
        };

        self.printer.archive(practice_static_dir, doc_bytes).await?;

        send_emails! {
            &self.mailer,
            MailTo {
                subject: "Práctica Autorizada".into(),
                template: "practice:authorization:secretary",
                email: self.mailer.context().config().secretary_email.clone(),
                context: context.clone(),
            },
            MailTo {
                subject: "Práctica Autorizada".into(),
                template: "practice:authorization:teacher",
                email: teacher.email.clone(),
                context
            }
        };

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

        send_emails! {
            &self.mailer,
            MailTo {
                subject: "Práctica Evaluada".into(),
                template: "practice:evaluation:teacher",
                email: teacher.email.clone(),
                context: email_context.clone(),
            },
            MailTo {
                subject: "Evaluación de Práctica Completada".into(),
                template: "practice:evaluation:supervisor",
                email: practice.supervisor_email.clone(),
                context: email_context,
            }
        };
    }

    pub async fn final_report_uploaded(
        &self,
        event: FinalReportUploadedEvent,
    ) -> ServiceResult<()> {
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

        send_emails! {
            &self.mailer,
            MailTo {
                subject: subject.clone(),
                template: "report-upload:teacher",
                email: teacher.email.clone(),
                context: ctx.clone(),
            },
            MailTo {
                subject,
                template: "report-upload:student",
                email: student.email.clone(),
                context: ctx,
            }
        };

        Ok(())
    }

    pub async fn meeting_request_created(
        &self,
        event: MeetingRequestCreatedEvent,
    ) -> ServiceResult<()> {
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

    pub async fn initialize_plagiarism_check(
        &self,
        event: InitializePlagiarismCheckEvent,
    ) -> ServiceResult<()> {
        let (practice_id, parsed_tex) = event;

        println!("\n🔍 ===== ANÁLISIS DE PLAGIO =====");
        println!("Práctica ID: {}", practice_id);

        // Analyze the document for plagiarism using the new service
        match self
            .plagiarism_service
            .analyze_document(practice_id, parsed_tex)
            .await
        {
            Ok(result) => {
                println!("\n✅ Análisis completado:");
                println!("   • Chunks analizados: {}", result.total_chunks_analyzed);
                println!("   • Matches encontrados: {}", result.total_matches_found);
                println!(
                    "   • Similitud máxima: {:.1}%",
                    result.max_similarity_score * 100.0
                );
                println!(
                    "   • Plagio significativo: {}",
                    if result.has_significant_plagiarism {
                        "SÍ ⚠️"
                    } else {
                        "NO ✓"
                    }
                );

                if !result.practice_summaries.is_empty() {
                    println!("\n📊 Top 3 prácticas más similares:");
                    for (i, summary) in
                        result.practice_summaries.iter().take(3).enumerate()
                    {
                        println!(
                            "   {}. Práctica {}: {} matches, avg={:.1}%, max={:.1}%, cobertura={:.1}%",
                            i + 1,
                            summary.practice_id,
                            summary.match_count,
                            summary.avg_similarity * 100.0,
                            summary.max_similarity * 100.0,
                            summary.coverage_percentage
                        );
                    }
                }

                if result.has_significant_plagiarism && !result.matches.is_empty() {
                    println!("\n⚠️  Matches de alta similitud:");
                    let high_similarity_matches: Vec<_> = result
                        .matches
                        .iter()
                        .filter(|m| m.similarity_score >= 0.90)
                        .take(5)
                        .collect();

                    for (i, m) in high_similarity_matches.iter().enumerate() {
                        println!(
                            "   {}. '{}' vs '{}' - {:.1}% (Práctica: {})",
                            i + 1,
                            m.source_section,
                            m.matched_section,
                            m.similarity_score * 100.0,
                            m.matched_practice_id
                        );
                    }
                }

                println!("\n===== FIN ANÁLISIS =====\n");
            }
            Err(e) => {
                println!("❌ Error en análisis de plagio: {}", e);
                // Don't fail the entire process, just log the error
            }
        }

        Ok(())
    }
}
