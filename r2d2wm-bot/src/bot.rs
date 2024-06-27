use crate::config::{MentionTarget, ScheduledMessage};
use crate::Result;
use chrono::Local;
use serenity::all::{
    ChannelId, Context, EventHandler, GuildId, MessageBuilder, Ready, RoleId, UserId,
};
use serenity::async_trait;
use serenity::builder::CreateMessage;
use serenity::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler};

pub async fn start(token: &str, schedule: Vec<ScheduledMessage>) -> Result<()> {
    let intents: GatewayIntents = GatewayIntents::non_privileged();

    let mut client: Client = Client::builder(token, intents)
        .event_handler(Handler::new(schedule))
        .await?;

    client.start().await?;
    tracing::info!("Client ready!");

    Ok(())
}

struct Handler {
    scheduled_messages: Vec<ScheduledMessage>,
}

impl Handler {
    pub fn new(scheduled_messages: Vec<ScheduledMessage>) -> Self {
        Self { scheduled_messages }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        tracing::info!("Cache ready! Starting schedulers spawns...");
        let ctx = Arc::from(ctx);

        let Ok(sched) = JobScheduler::new().await else {
            tracing::error!("Failed to create scheduler");
            return;
        };

        for message in &self.scheduled_messages {
            tracing::debug!("Spawning scheduler for {:?}", &message.name);
            let message = Arc::new(message.clone());
            let ctx = Arc::clone(&ctx);
            let job = Job::new_async(format!("0 {}", message.cron).as_str(), move |_uuid, _l| {
                let ctx = Arc::clone(&ctx);
                let message = Arc::clone(&message);
                Box::pin(async move {
                    send_message(Arc::clone(&ctx), Arc::clone(&message)).await;
                })
            })
            .unwrap();
            sched.add(job).await.unwrap();
        }

        sched.start().await.unwrap();
    }

    async fn ready(&self, _: Context, ready: Ready) {
        tracing::info!("{} is connected!", ready.user.name);
    }
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
