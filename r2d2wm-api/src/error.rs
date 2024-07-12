use rocket::Responder;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Responder)]
pub enum Error {
    #[response(status = 500)]
    Internal(String),
    #[response(status = 503)]
    ServiceUnavailable(String),
    #[response(status = 400)]
    BadQuery(String),
    #[response(status = 404)]
    NotFound(String),
}
