use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Cannot respond to slash command")]
    CommandResponse(#[source] serenity::Error),
    #[error("Cannot create slash command")]
    CreateCommand(#[source] serenity::Error),
    #[error("Cannot create scheduler")]
    CreateScheduler(#[source] tokio_cron_scheduler::JobSchedulerError),
    #[error("Cannot create cron job: {1}")]
    CreateCronJob(#[source] tokio_cron_scheduler::JobSchedulerError, String),
    #[error("Cannot create multiple cron jobs")]
    CannotCreateMultipleCronJob(Vec<Error>),
    #[error("Cannot parse cron expression")]
    ParseCronExpr(#[source] tokio_cron_scheduler::JobSchedulerError),
    #[error(transparent)]
    ParseTimezone(#[from] chrono_tz::ParseError),
    #[error(transparent)]
    EnvVar(#[from] std::env::VarError),
    #[error(transparent)]
    Serenity(#[from] serenity::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    SerializeOrDeserializeJson(#[from] serde_json::Error),
}
