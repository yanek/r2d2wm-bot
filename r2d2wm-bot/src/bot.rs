use crate::config::ScheduledMessage;
use crate::Result;
use chrono::{DateTime, Local};
use croner::Cron;
use serenity::all::{ChannelId, Context, EventHandler, GuildId, MessageBuilder, Ready};
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
    is_loop_running: AtomicBool,
    scheduled_messages: Vec<ScheduledMessage>,
}

impl Handler {
    pub fn new(scheduled_messages: Vec<ScheduledMessage>) -> Self {
        Self {
            is_loop_running: AtomicBool::default(),
            scheduled_messages,
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        let ctx: Arc<Context> = Arc::from(ctx);

        if !self.is_loop_running.load(Ordering::Relaxed) {
            for message in &self.scheduled_messages {
                tokio::spawn(run_task(Arc::clone(&ctx), message.clone()));
            }

            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        tracing::info!("{} is connected!", ready.user.name);
    }
}

async fn run_task(ctx: Arc<Context>, msg_data: ScheduledMessage) -> Result<()> {
    let cron: Cron = Cron::new(&msg_data.cron).parse()?;
    let channel: &ChannelId = &msg_data.channel_id;
    let message: CreateMessage = build_message(&msg_data);

    loop {
        let current_time: DateTime<Local> = Local::now();
        if cron.is_time_matching(&current_time)? {
            channel.send_message(&ctx.http, message.clone()).await?;
            tracing::debug!("Message sent: {:?}", &msg_data);
            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

fn build_message(msg_data: &ScheduledMessage) -> CreateMessage {
    let mut msg_builder: MessageBuilder = MessageBuilder::new();
    msg_builder.push(&msg_data.message);

    if let Some(recipients) = &msg_data.recipients {
        msg_builder.push_line("");

        for (i, role_id) in recipients.iter().enumerate() {
            msg_builder.mention(role_id);
            if i < recipients.len() - 1 {
                msg_builder.push(" ");
            }
        }
    }

    CreateMessage::new().content(msg_builder.build())
}
