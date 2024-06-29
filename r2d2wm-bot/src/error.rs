use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot create new scheduler: {0}")]
    CannotCreateScheduler(#[from] tokio_cron_scheduler::JobSchedulerError),

    #[error("Cannot create cron job: {0}")]
    CannotCreateCronJob(String),

    #[error("Cannot create multiple cron jobs")]
    CannotCreateMultipleCronJob(Vec<Error>),

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
