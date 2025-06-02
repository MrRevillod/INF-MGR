#[derive(Debug)]
pub enum UserError {
    NotFound,
    EmailAlreadyExists,
    InvalidEmail,
    IdAlreadyExists,
    UnexpectedError(String),
    InvalidRole,
}
