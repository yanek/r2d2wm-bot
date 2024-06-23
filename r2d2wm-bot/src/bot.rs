use crate::handler::Handler;
use crate::Result;
use serenity::prelude::*;
use std::env;

pub async fn run() -> Result<()> {
    let token = env::var("DISCORD_TOKEN")?;
    let intents = GatewayIntents::non_privileged();

    tracing::info!("Creating client...");
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await?;
    tracing::info!("Client ready");

    client.start().await?;
    Ok(())
}
