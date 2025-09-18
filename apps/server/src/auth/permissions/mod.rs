use crate::shared::errors::AuthError;
use std::fmt::Display;

mod middleware;
pub use middleware::RequirePermission;

mod service;
pub use service::Permissions;

#[derive(Clone)]
pub struct Permission {
    pub resource: String,
    pub action: String,
    pub scope: Option<PermissionScope>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PermissionScope {
    All,     // e.g., Admin
    Own,     // e.g., Teacher's own courses
    Related, // e.g., Student's enrolled courses
}

impl Permission {
    pub fn matches_without_scope(&self, other: &Permission) -> bool {
        if self.resource != other.resource || self.action != other.action {
            return false;
        }

        true
    }
}

impl TryFrom<&str> for Permission {
    type Error = AuthError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(':').collect();

        match parts.len() {
            2 => Ok(Permission {
                resource: parts[0].to_string(),
                action: parts[1].to_string(),
                scope: None,
            }),
            3 => Ok(Permission {
                resource: parts[0].to_string(),
                action: parts[1].to_string(),
                scope: Some(PermissionScope::try_from(parts[2])?),
            }),
            _ => Err(AuthError::Other(format!(
                "Invalid permission format: {value}"
            ))),
        }
    }
}

impl Display for PermissionScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scope_str = match self {
            PermissionScope::All => "all",
            PermissionScope::Own => "own",
            PermissionScope::Related => "related",
        };

        write!(f, "{}", scope_str)
    }
}

impl Display for Permission {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(scope) = &self.scope {
            write!(f, "{}:{}:{}", self.resource, self.action, scope)
        } else {
            write!(f, "{}:{}", self.resource, self.action)
        }
    }
}

impl TryFrom<&str> for PermissionScope {
    type Error = AuthError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "all" => Ok(PermissionScope::All),
            "own" => Ok(PermissionScope::Own),
            "related" => Ok(PermissionScope::Related),
            _ => Err(AuthError::Other(format!(
                "Invalid permission scope: {value}"
            ))),
        }
    }
}
