#[derive(Debug)]
pub enum ServiceError {
    Hasher(String),
    Mailer(String),
}
