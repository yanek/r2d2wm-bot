use derive_more::From;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    CannotParseCron(croner::errors::CronError),
    #[from]
    CannotParseTimezone(chrono_tz::ParseError),
    #[from]
    EnvVar(std::env::VarError),
    #[from]
    Serenity(serenity::Error),
    #[from]
    Io(std::io::Error),
    #[from]
    CannotSerializeOrDeserializeJson(serde_json::Error),
}

impl std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
