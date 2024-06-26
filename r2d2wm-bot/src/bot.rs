use crate::config::{MentionTarget, ScheduledMessage};
use crate::Result;
use chrono::{DateTime, Local};
use croner::Cron;
use serenity::all::{
    ChannelId, Context, EventHandler, GuildId, MessageBuilder, Ready, RoleId, UserId,
};
use serenity::async_trait;
use serenity::builder::CreateMessage;
use serenity::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub async fn start(token: &str, schedule: Vec<ScheduledMessage>) -> Result<()> {
    let intents: GatewayIntents = GatewayIntents::non_privileged();

    let mut client: Client = Client::builder(token, intents)
        .event_handler(Handler::new(schedule))
        .await?;

    client.start().await?;
    tracing::info!("Client ready");

    Ok(())
}

struct Handler {
    is_scheduler_running: AtomicBool,
    scheduled_messages: Vec<ScheduledMessage>,
}

impl Handler {
    pub fn new(scheduled_messages: Vec<ScheduledMessage>) -> Self {
        Self {
            is_scheduler_running: AtomicBool::default(),
            scheduled_messages,
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        let ctx: Arc<Context> = Arc::from(ctx);
        tracing::trace!("Cache is ready");

        if !self.is_scheduler_running.load(Ordering::Relaxed) {
            for message in &self.scheduled_messages {
                tokio::spawn(run_task(Arc::clone(&ctx), message.clone()));
            }
            self.is_scheduler_running.swap(true, Ordering::Relaxed);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        tracing::info!("{} is connected!", ready.user.name);
    }
}

async fn run_task(ctx: Arc<Context>, msg_data: ScheduledMessage) {
    let Ok(cron) = Cron::new(&msg_data.cron).parse() else {
        tracing::error!("Failed to parse cron expression: {:?}", &msg_data.cron);
        return;
    };

    let channel: ChannelId = ChannelId::new(msg_data.channel_id.get());
    let message: CreateMessage = build_message(&msg_data);

    loop {
        let current_time: DateTime<Local> = Local::now();
        let Ok(is_time_matching) = cron.is_time_matching(&current_time) else {
            tracing::error!("Failed to check if current time matches cron job");
            continue;
        };

        if is_time_matching {
            match channel.send_message(&ctx.http, message.clone()).await {
                Ok(msg) => {
                    tracing::info!("Message sent on schedule: {:?}", &msg.id);
                    tracing::trace!("{:?}", &msg);
                    //TODO: remove this, and wrap the message into something with a state
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                Err(e) => {
                    tracing::error!("Failed to send message: {:?}: {:?}", &msg_data.name, e);
                }
            }
        }

        tokio::time::sleep(Duration::from_millis(10)).await;
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
