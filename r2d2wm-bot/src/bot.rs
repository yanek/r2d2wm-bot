use std::sync::Arc;

use anyhow::Result;
use chrono_tz::Tz;
use serenity::all::{Context, EventHandler, GatewayIntents, GuildId, Interaction, Ready};
use serenity::async_trait;
use serenity::Client;

use r2d2wm_core::Task;

use crate::command;
use crate::scheduler::{persistence, Scheduler};

pub async fn start(token: &str, timezone: Tz) -> Result<()> {
    let intents: GatewayIntents = GatewayIntents::non_privileged();
    let schedule = persistence::get_all_messages().await?;

    let mut client: Client = Client::builder(token, intents)
        .event_handler(Handler::new(schedule, timezone))
        .await?;

    client.start().await?;

    Ok(())
}

struct Handler {
    scheduled_messages: Vec<Task>,
    timezone: Tz,
}

impl Handler {
    pub fn new(scheduled_messages: Vec<Task>, timezone: Tz) -> Self {
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

        command::register_all(&ctx.http)
            .await
            .iter()
            .for_each(|result| match result {
                Ok(command) => {
                    let name = &command.name;
                    tracing::info!("Registered slash command: {name:?}");
                }
                Err(e) => tracing::error!("{e}: {e:?}"),
            });
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let Interaction::Command(command) = interaction else {
            return;
        };

        match command::run(&ctx, &command).await {
            Ok(()) => {}
            Err(e) => tracing::error!("{e}: {e:?}"),
        };
    }
}
