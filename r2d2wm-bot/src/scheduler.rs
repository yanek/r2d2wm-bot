use std::sync::Arc;

use chrono_tz::Tz;
use serenity::prelude::Context;
use tokio_cron_scheduler::{Job, JobBuilder, JobScheduler, JobToRunAsync};

use crate::config::ScheduledMessage;
use crate::{Error, Result};

#[allow(clippy::struct_field_names)]
pub struct Scheduler {
    internal_scheduler: JobScheduler,
    timezone: Tz,
    discord_context: Arc<Context>,
}

impl Scheduler {
    pub async fn new(discord_context: Arc<Context>, timezone: Tz) -> Result<Self> {
        let internal_scheduler = JobScheduler::new().await.map_err(Error::CreateScheduler)?;

        internal_scheduler
            .start()
            .await
            .map_err(Error::CreateScheduler)?;

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
            .map_err(|e| Error::CreateCronJob(e, message.name.clone()))?;
        tracing::info!("Spawned job for {:?} (uuid={:?})", message.name, uuid);
        Ok(())
    }

    pub async fn push_many(&self, messages: Vec<ScheduledMessage>) -> Vec<Result<()>> {
        let mut results = Vec::new();
        for message in &messages {
            let res = self.push(message.clone()).await;
            results.push(res);
        }

        results
    }

    fn create_cron_job(&self, message: Arc<ScheduledMessage>) -> Result<Job> {
        let ctx = Arc::clone(&self.discord_context);
        let name = message.name.clone();
        let job = JobBuilder::new()
            .with_schedule(format!("0 {}", message.cron).as_str())
            .map_err(Error::ParseCronExpr)?
            .with_timezone(self.timezone)
            .with_cron_job_type()
            .with_run_async(Self::send_message_async(message, ctx))
            .build()
            .map_err(|e| Error::CreateCronJob(e, name))?;
        Ok(job)
    }

    fn send_message_async(message: Arc<ScheduledMessage>, ctx: Arc<Context>) -> Box<JobToRunAsync> {
        Box::new(move |_uuid, _l| {
            let ctx = ctx.clone();
            let message = Arc::clone(&message);
            Box::pin(async move {
                message.send_to_discord(ctx.clone()).await;
            })
        })
    }
}
