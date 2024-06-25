use crate::config::ScheduledMessage;
use crate::Result;
use chrono_tz::Tz;
use std::collections::HashMap;
use std::str::FromStr;
use tokio_cron_scheduler::{Job, JobBuilder, JobScheduler};
use uuid::Uuid;

pub struct MessageScheduler {
    running_jobs: HashMap<String, Uuid>,
    job_scheduler: JobScheduler,
    timezone: Tz,
}

impl MessageScheduler {
    pub async fn new_and_run(timezone: Tz, messages: &Vec<ScheduledMessage>) -> Result<Self> {
        let mut scheduler = Self {
            running_jobs: HashMap::new(),
            job_scheduler: JobScheduler::new().await?,
            timezone,
        };

        for msg in messages {
            scheduler.add(msg).await?;
        }

        scheduler.job_scheduler.start().await?;
        Ok(scheduler)
    }

    pub async fn add(&mut self, scheduled_message: &ScheduledMessage) -> Result<()> {
        let cron = cron::Schedule::from_str(&format!("0 {}", scheduled_message.cron))?;
        let msg = scheduled_message.message.clone();
        let job: Job = JobBuilder::new()
            .with_timezone(self.timezone)
            .with_cron_job_type()
            .with_schedule(cron)?
            .with_run_sync(Box::new(move |_, _| {
                tracing::info!("{}", msg);
            }))
            .build()?;

        let key = scheduled_message.name.clone();
        self.running_jobs.insert(key, job.guid());
        self.job_scheduler.add(job).await?;

        Ok(())
    }

    pub async fn remove(&mut self, name: &str) -> Result<()> {
        if let Some(guid) = self.running_jobs.remove(name) {
            self.job_scheduler.remove(&guid).await?;
            tracing::info!("Removed job: {}", name);
        } else {
            tracing::warn!("Job not found: {}", name);
        }

        tracing::warn!("Job not found: {}", name);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_manager() -> Result<MessageScheduler> {
        let messages = Vec::new();
        MessageScheduler::new_and_run(Tz::Europe__Paris, &messages).await
    }

    fn setup_message() -> ScheduledMessage {
        ScheduledMessage {
            name: "test".to_string(),
            cron: "* * * * *".to_string(),
            recipients: None,
            message: "test message".to_string(),
        }
    }

    #[tokio::test]
    async fn test_add() {
        let mut manager = setup_manager()
            .await
            .expect("Failed to setup RunningJobManager");

        let scheduled_message = setup_message();

        manager
            .add(&scheduled_message)
            .await
            .expect("Failed to add job");
        assert!(manager.running_jobs.contains_key(&scheduled_message.name));
    }

    #[tokio::test]
    async fn test_remove() {
        let mut manager = setup_manager()
            .await
            .expect("Failed to setup RunningJobManager");

        let scheduled_message = setup_message();

        manager
            .add(&scheduled_message)
            .await
            .expect("Failed to add job");
        manager
            .remove(&scheduled_message.name)
            .await
            .expect("Failed to remove job");
        assert!(!manager.running_jobs.contains_key(&scheduled_message.name));
    }
}
