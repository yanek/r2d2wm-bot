use crate::config::ScheduledMessage;
use crate::Result;
use serenity::all::{Context, EventHandler, GuildId, MessageBuilder, Ready};
use serenity::async_trait;
use serenity::builder::CreateMessage;
use serenity::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

pub struct Bot {
    token: String,
}

impl Bot {
    pub fn new(token: &str) -> Self {
        Bot {
            token: token.to_string(),
        }
    }

    pub async fn start(&self, schedule: Vec<ScheduledMessage>) -> Result<()> {
        let intents = GatewayIntents::non_privileged();
        let mut client = Client::builder(&self.token, intents)
            .event_handler(Handler::new(schedule))
            .await?;
        client.start().await?;
        tracing::info!("Client ready");
        Ok(())
    }
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

    async fn run_task(ctx: Arc<Context>, msg_data: ScheduledMessage) -> Result<()> {
        let cron = croner::Cron::new(&msg_data.cron).parse()?;
        let channel = &msg_data.channel_id;

        let mut msg_builder = MessageBuilder::new();
        msg_builder.push(msg_data.message);
        if let Some(recipients) = msg_data.recipients {
            for role_id in &recipients {
                msg_builder.mention(role_id);
            }
            msg_builder.push_line("");
        }

        let message = CreateMessage::new().content(msg_builder.build());
        loop {
            let current_time = chrono::Local::now();
            if cron.is_time_matching(&current_time)? {
                channel.send_message(&ctx.http, message.clone()).await?;
                tokio::time::sleep(Duration::from_secs(1)).await;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        let ctx = Arc::from(ctx);

        if !self.is_loop_running.load(Ordering::Relaxed) {
            for message in &self.scheduled_messages {
                tokio::spawn(Self::run_task(Arc::clone(&ctx), message.clone()));
            }

            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        tracing::info!("{} is connected!", ready.user.name);
    }
}
