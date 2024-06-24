use crate::config::Config;
use crate::handler::Handler;
use crate::Result;
use serenity::prelude::*;

pub async fn run(config: &Config) -> Result<()> {
    let intents = GatewayIntents::non_privileged();

    tracing::info!("Creating client...");
    let mut client = Client::builder(&config.app.discord_token, intents)
        .event_handler(Handler)
        .await?;
    tracing::info!("Client ready");

    client.start().await?;
    Ok(())
}
