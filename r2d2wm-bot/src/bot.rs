use crate::config::{MentionTarget, ScheduledMessage};
use crate::Error;
use crate::Result;
use chrono_tz::Tz;
use serenity::all::{
    ChannelId, Context, EventHandler, GuildId, MessageBuilder, Ready, RoleId, UserId,
};
use serenity::async_trait;
use serenity::builder::CreateMessage;
use serenity::prelude::*;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobBuilder, JobScheduler, JobSchedulerError};

pub async fn start(token: &str, timezone: Tz, schedule: Vec<ScheduledMessage>) -> Result<()> {
    let intents: GatewayIntents = GatewayIntents::non_privileged();

    let mut client: Client = Client::builder(token, intents)
        .event_handler(Handler::new(schedule, timezone))
        .await?;

    client.start().await?;

    Ok(())
}

struct Handler {
    scheduled_messages: Vec<ScheduledMessage>,
    timezone: Tz,
}

impl Handler {
    pub fn new(scheduled_messages: Vec<ScheduledMessage>, timezone: Tz) -> Self {
        Self {
            scheduled_messages,
            timezone,
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        tracing::info!("Cache ready! Starting cron jobs spawn...");
        let ctx = Arc::from(ctx);

        let Ok(sched) = JobScheduler::new().await else {
            tracing::error!("Failed to create scheduler");
            return;
        };

        for message in &self.scheduled_messages {
            let message = Arc::new(message.clone());
            let ctx = Arc::clone(&ctx);

            match push_job(&sched, Arc::clone(&message), ctx, self.timezone).await {
                Ok(()) => {
                    tracing::info!("Spawned job for {:?}", &message.name);
                }
                Err(e) => {
                    tracing::error!("Failed to create job: {e:?}");
                    continue;
                }
            }
        }

        match sched.start().await {
            Ok(_) => {
                tracing::info!("Successfully started scheduler!")
            }
            Err(e) => {
                tracing::error!("Failed to start scheduler: {e:?}")
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        tracing::info!("{} is online!", ready.user.name);
    }
}

async fn push_job(
    schedule: &JobScheduler,
    message: Arc<ScheduledMessage>,
    ctx: Arc<Context>,
    timezone: Tz,
) -> Result<()> {
    let job = create_cron_job(message, ctx, timezone)?;
    schedule
        .add(job)
        .await
        .map_err(|e| Error::CannotCreateCronJob(format!("Failed to create job: {e}")))?;
    Ok(())
}

fn create_cron_job(message: Arc<ScheduledMessage>, ctx: Arc<Context>, timezone: Tz) -> Result<Job> {
    let job = JobBuilder::new()
        .with_schedule(format!("0 {}", message.cron).as_str())
        .map_err(|_| Error::CannotCreateCronJob("Failed to parse cron expression".to_string()))?
        .with_timezone(timezone)
        .with_cron_job_type()
        .with_run_async(Box::new(move |_uuid, _l| {
            let ctx = Arc::clone(&ctx);
            let message = Arc::clone(&message);
            Box::pin(async move {
                send_message(Arc::clone(&ctx), Arc::clone(&message)).await;
            })
        }))
        .build()
        .map_err(|_| Error::CannotCreateCronJob("Failed to build".to_string()))?;

    Ok(job)
}

async fn send_message(ctx: Arc<Context>, msg_data: Arc<ScheduledMessage>) {
    let channel: ChannelId = ChannelId::new(msg_data.channel_id.get());
    let message: CreateMessage = build_message(&msg_data);

    match channel.send_message(&ctx.http, message.clone()).await {
        Ok(msg) => {
            tracing::info!("Message sent on schedule: {:?}", &msg.id);
            tracing::trace!("{:?}", &msg);
        }
        Err(e) => {
            tracing::error!("Failed to send message: {:?}: {:?}", &msg_data.name, e);
        }
    }
}

fn build_message(msg_data: &ScheduledMessage) -> CreateMessage {
    let mut msg_builder: MessageBuilder = MessageBuilder::new();
    msg_builder.push(&msg_data.message);

    if let Some(mentions) = &msg_data.mentions {
        msg_builder.push_line("");

        for (i, target) in mentions.iter().enumerate() {
            match target {
                MentionTarget::Role(id) => msg_builder.mention(&RoleId::new(id.get())),
                MentionTarget::User(id) => msg_builder.mention(&UserId::new(id.get())),
            };
            if i < mentions.len() - 1 {
                msg_builder.push(" ");
            }
        }
    }

    CreateMessage::new().content(msg_builder.build())
}
