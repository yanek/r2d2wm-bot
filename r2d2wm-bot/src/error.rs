use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[from("cannot create cron job: {0}")]
    CannotCreateCronJob(String),

    #[error(transparent)]
    CannotParseTimezone(#[from] chrono_tz::ParseError),

    #[error(transparent)]
    EnvVar(#[from] std::env::VarError),

    #[error(transparent)]
    Serenity(#[from] serenity::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    CannotSerializeOrDeserializeJson(#[from] serde_json::Error),
}
