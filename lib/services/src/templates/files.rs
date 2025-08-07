use std::sync::LazyLock;

use crate::templates::Templates;

pub static PRINTER_TEMPLATES: LazyLock<Templates> = LazyLock::new(|| {
    vec![(
        "document:practice:authorization",
        include_str!("../printer/templates/authorization.typ"),
    )]
});

pub static MAILER_TEMPLATES: LazyLock<Templates> = LazyLock::new(|| {
    vec![
        (
            "system:welcome.html",
            include_str!("../mailer/templates/system/welcome.html"),
        ),
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
            "practice:approval:coordinator.html",
            include_str!("../mailer/templates/practice/approval/coordinator.html"),
        ),
        (
            "practice:approval:student.html",
            include_str!("../mailer/templates/practice/approval/student.html"),
        ),
        (
            "practice:rejection:coordinator.html",
            include_str!("../mailer/templates/practice/rejection/coordinator.html"),
        ),
        (
            "practice:rejection:student.html",
            include_str!("../mailer/templates/practice/rejection/student.html"),
        ),
    ]
});
