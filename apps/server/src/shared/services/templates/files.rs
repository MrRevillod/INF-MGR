use std::sync::LazyLock;

use crate::shared::services::templates::Templates;

pub static PRINTER_TEMPLATES: LazyLock<Templates> = LazyLock::new(|| {
    vec![(
        "document:practice:authorization",
        include_str!("../printer/templates/authorization.typ"),
    )]
});

pub static MAILER_TEMPLATES: LazyLock<Templates> = LazyLock::new(|| {
    vec![
        ("system:welcome.html", include_str!("../mailer/templates/system/welcome.html")),
        (
            "practice:creation:supervisor.html",
            include_str!("../mailer/templates/practice/creation/supervisor.html"),
        ),
        (
            "practice:creation:student.html",
            include_str!("../mailer/templates/practice/creation/student.html"),
        ),
        (
            "practice:approval:supervisor.html",
            include_str!("../mailer/templates/practice/approval/supervisor.html"),
        ),
        (
            "practice:approval:teacher.html",
            include_str!("../mailer/templates/practice/approval/teacher.html"),
        ),
        (
            "practice:approval:student.html",
            include_str!("../mailer/templates/practice/approval/student.html"),
        ),
        (
            "practice:approval:secretary.html",
            include_str!("../mailer/templates/practice/approval/secretary.html"),
        ),
        (
            "practice:rejection:teacher.html",
            include_str!("../mailer/templates/practice/rejection/teacher.html"),
        ),
        (
            "practice:rejection:student.html",
            include_str!("../mailer/templates/practice/rejection/student.html"),
        ),
        (
            "course:creation:teacher.html",
            include_str!("../mailer/templates/course/creation/teacher.html"),
        ),
    ]
});
