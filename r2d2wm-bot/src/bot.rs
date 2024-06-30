use std::sync::Arc;

use chrono_tz::Tz;
use serenity::all::{Context, EventHandler, GatewayIntents, GuildId, Ready};
use serenity::{async_trait, Client};

use crate::config::ScheduledMessage;
use crate::scheduler::Scheduler;
use crate::Result;

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
            .for_each(|job| match job {
                Ok(()) => {}
                Err(e) => tracing::error!("{e}: {e:?}"),
            });
    }

    async fn ready(&self, _: Context, ready: Ready) {
        tracing::info!("{} is online!", ready.user.name);
    }
}
