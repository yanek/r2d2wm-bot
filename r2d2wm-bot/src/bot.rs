use crate::config::Config;
use crate::Result;
use serenity::all::{Context, EventHandler, Ready};
use serenity::async_trait;
use serenity::prelude::*;

pub async fn run(config: &Config) -> Result<()> {
    let intents = GatewayIntents::non_privileged();

    tracing::debug!("Creating client...");
    let mut client = Client::builder(&config.app.discord_token, intents)
        .event_handler(Handler)
        .await?;
    tracing::info!("Client ready");

    client.start().await?;
    Ok(())
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        tracing::info!("{} is connected!", ready.user.name);
    }
}
