use std::sync::Arc;

use crate::scheduler::message::send_to_discord;
use anyhow::{Context, Result};
use chrono_tz::Tz;
use r2d2wm_core::Task;
use serenity::all::Context as SerenityContext;
use tokio_cron_scheduler::{Job, JobBuilder, JobScheduler, JobToRunAsync};

mod message;
pub mod persistence;

pub struct Scheduler {
    internal: JobScheduler,
    timezone: Tz,
    discord_context: Arc<SerenityContext>,
}

impl Scheduler {
    pub async fn new(discord_context: Arc<SerenityContext>, timezone: Tz) -> Result<Self> {
        let internal = JobScheduler::new().await?;
        internal.start().await?;

        Ok(Scheduler {
            internal,
            timezone,
            discord_context,
        })
    }

    pub async fn push(&self, message: Task) -> Result<()> {
        let message: Arc<Task> = Arc::from(message);
        let job = self.create_cron_job(Arc::clone(&message))?;
        let uuid = job.guid();
        self.internal
            .add(job)
            .await
            .context(format!("Cannot create job: {}", &message.name))?;
        tracing::info!("Spawned job for {:?} (uuid={:?})", message.name, uuid);
        Ok(())
    }

    pub async fn push_many(&self, messages: Vec<Task>) -> Vec<Result<()>> {
        let mut results = Vec::new();
        for message in &messages {
            let res = self.push(message.clone()).await;
            results.push(res);
        }

        results
    }

    fn create_cron_job(&self, message: Arc<Task>) -> Result<Job> {
        let ctx = Arc::clone(&self.discord_context);
        let job = JobBuilder::new()
            .with_schedule(format!("0 {}", message.cron_expr).as_str())?
            .with_timezone(self.timezone)
            .with_cron_job_type()
            .with_run_async(Self::send_message_async(message, ctx))
            .build()?;
        Ok(job)
    }

    fn send_message_async(tsk: Arc<Task>, ctx: Arc<SerenityContext>) -> Box<JobToRunAsync> {
        Box::new(move |_uuid, _l| {
            let ctx = ctx.clone();
            let message = Arc::clone(&tsk);
            Box::pin(async move {
                send_to_discord(&message, ctx.clone()).await;
            })
        })
    }
}
