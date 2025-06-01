#[derive(Debug)]
pub enum UserError {
    NotFound,
    EmailAlreadyExists,
    UsernameAlreadyExists,
    UnexpectedError,
    InvalidEmail,
    InvalidId,
}
