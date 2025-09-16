use crate::users::Role;
use std::collections::HashSet;

pub struct Permissions;

impl Permissions {
    pub fn build(roles: &Vec<Role>) -> Vec<String> {
        let mut permissions = HashSet::new();

        let permissions_for_role = |role: &Role| -> Vec<String> {
            match role {
                Role::Administrator => Self::build_admin_permissions(),
                Role::Secretary => Self::build_secretary_permissions(),
                Role::Teacher => Self::build_teacher_permissions(),
                Role::Student => Self::build_student_permissions(),
            }
        };

        roles.iter().for_each(|role| {
            let role_permissions = permissions_for_role(role);

            role_permissions.into_iter().for_each(|perm| {
                permissions.insert(perm);
            });
        });

        permissions.into_iter().collect()
    }

    fn build_admin_permissions() -> Vec<String> {
        vec!["*".to_string()]
    }

    fn build_secretary_permissions() -> Vec<String> {
        vec![
            // Courses management
            "courses:read:all".to_string(),
            "courses:create".to_string(),
            "courses:update:all".to_string(),
            "courses:delete".to_string(),
            // Enrollments management
            "enrollments:read:all".to_string(),
            "enrollments:create".to_string(),
            "enrollments:delete".to_string(),
            // Users management
            "users:read:all".to_string(),
            "users:create:all".to_string(),
            "users:update:all".to_string(),
            "users:delete:all".to_string(),
        ]
    }

    fn build_teacher_permissions() -> Vec<String> {
        vec![
            // Own profile
            "users:read:own".to_string(),
            // Own courses
            "courses:read:own".to_string(),
            "courses:update:own".to_string(),
            // Related enrollments (students in their courses)
            "enrollments:read:related".to_string(),
            "enrollments:update:related".to_string(),
        ]
    }

    fn build_student_permissions() -> Vec<String> {
        vec![
            // Own profile
            "users:read:own".to_string(),
            // Related courses (enrolled courses)
            "courses:read:related".to_string(),
            // Own enrollments
            "enrollments:read:own".to_string(),
        ]
    }
}
