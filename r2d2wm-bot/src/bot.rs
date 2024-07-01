use std::sync::Arc;

use chrono_tz::Tz;
use serenity::all::{
    Context, CreateInteractionResponse, CreateInteractionResponseMessage, EventHandler,
    GatewayIntents, GuildId, Interaction, Ready,
};
use serenity::async_trait;
use serenity::Client;

use crate::config::ScheduledMessage;
use crate::scheduler::Scheduler;
use crate::{command, Error, Result};

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
        let sched = match Scheduler::new(ctx, self.timezone).await {
            Ok(sched) => {
                tracing::info!("Successfully started a new scheduler");
                sched
            }
            Err(e) => {
                tracing::error!("{e}: {e:?}");
                return;
            }
        };

        sched
            .push_many(self.scheduled_messages.clone())
            .await
            .iter()
            .for_each(|result| match result {
                Ok(()) => {}
                Err(e) => tracing::error!("{e}: {e:?}"),
            });
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        tracing::info!("{} is online!", ready.user.name);

        command::register_commands(&ctx.http)
            .await
            .iter()
            .for_each(|result| match result {
                Ok(command) => {
                    let name = &command.name;
                    tracing::info!("Registered slash command: {name}");
                }
                Err(e) => tracing::error!("{e}: {e:?}"),
            });
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let Interaction::Command(command) = interaction else {
            return;
        };

        let content = match command.data.name.as_str() {
            "ping" => Some(command::ping::run(&command.data.options())),
            _ => None,
        };

        if let Some(content) = content {
            let data = CreateInteractionResponseMessage::new().content(content);
            let builder = CreateInteractionResponse::Message(data);
            if let Err(e) = command
                .create_response(&ctx.http, builder)
                .await
                .map_err(Error::CommandResponse)
            {
                tracing::error!("{e}: {e:?}");
            }
        }

        tracing::debug!("Received command: {command:?}");
    }
}
