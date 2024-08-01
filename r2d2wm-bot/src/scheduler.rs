use std::sync::Arc;

use crate::scheduler::message::send_to_discord;
use anyhow::{Context, Result};
use chrono_tz::Tz;
use poise::serenity_prelude as serenity;
use r2d2wm_core::Task;
use tokio_cron_scheduler::{Job, JobBuilder, JobScheduler};

mod message;
pub mod persistence;

pub struct Scheduler {
    internal: JobScheduler,
    timezone: Tz,
    discord_context: serenity::Context,
}

impl Scheduler {
    pub async fn new(discord_context: serenity::Context, timezone: Tz) -> Result<Self> {
        let internal = JobScheduler::new().await?;
        internal.start().await?;

        Ok(Scheduler {
            internal,
            timezone,
            discord_context,
        })
    }

    pub async fn push(&self, message: Task) -> Result<()> {
        let job = self.create_cron_job(message)?;
        let guid = job.guid();

        self.internal
            .add(job)
            .await
            .context(format!("Cannot create job: {}.", &guid))?;
        tracing::info!("Added job: {}.", &guid);
        Ok(())
    }

    fn create_cron_job(&self, message: Task) -> Result<Job> {
        let ctx = Arc::from(self.discord_context.clone());

        let job = JobBuilder::new()
            .with_schedule(format!("0 {}", message.cron_expr).as_str())?
            .with_timezone(self.timezone)
            .with_cron_job_type()
            .with_run_async(Box::new(move |_uuid, _l| {
                let msg = message.clone();
                let ctx = Arc::clone(&ctx);

                Box::pin(async move {
                    send_to_discord(msg, ctx).await;
                })
            }))
            .build()?;
        Ok(job)
    }
}
