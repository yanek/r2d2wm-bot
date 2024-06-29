use crate::config::ScheduledMessage;
use crate::{Error, Result};
use chrono_tz::Tz;
use serenity::prelude::Context;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobBuilder};

#[allow(clippy::struct_field_names)]
pub struct Scheduler {
    internal_scheduler: tokio_cron_scheduler::JobScheduler,
    timezone: Tz,
    discord_context: Arc<Context>,
}

impl Scheduler {
    pub async fn new(discord_context: Arc<Context>, timezone: Tz) -> Result<Self> {
        let internal_scheduler = tokio_cron_scheduler::JobScheduler::new().await?;
        internal_scheduler.start().await?;

        Ok(Scheduler {
            internal_scheduler,
            timezone,
            discord_context,
        })
    }

    pub async fn push(&self, message: ScheduledMessage) -> Result<()> {
        let message: Arc<ScheduledMessage> = Arc::from(message);
        let job = self.create_cron_job(Arc::clone(&message))?;
        let uuid = job.guid();
        self.internal_scheduler
            .add(job)
            .await
            .map_err(|e| Error::CannotCreateCronJob(format!("Failed to create job: {e}")))?;
        tracing::info!("Spawned job for {}: uuid={}", message.name, uuid);
        Ok(())
    }

    pub async fn push_many(&self, messages: Vec<ScheduledMessage>) -> Result<()> {
        let mut errors = Vec::new();
        for message in &messages {
            match self.push(message.clone()).await {
                Ok(()) => {}
                Err(e) => {
                    errors.push(e);
                }
            }
        }

        if !errors.is_empty() {
            return Err(Error::CannotCreateMultipleCronJob(errors));
        }

        Ok(())
    }

    fn create_cron_job(&self, message: Arc<ScheduledMessage>) -> Result<Job> {
        let ctx = Arc::clone(&self.discord_context);
        let job = JobBuilder::new()
            .with_schedule(format!("0 {}", message.cron).as_str())
            .map_err(|_| Error::CannotCreateCronJob("Failed to parse cron expression".to_string()))?
            .with_timezone(self.timezone)
            .with_cron_job_type()
            .with_run_async(Box::new(move |_uuid, _l| {
                let ctx = ctx.clone();
                let message = Arc::clone(&message);
                Box::pin(async move {
                    message.send(ctx.clone()).await;
                })
            }))
            .build()
            .map_err(|_| Error::CannotCreateCronJob("Failed to build".to_string()))?;
        Ok(job)
    }
}
