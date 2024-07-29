use std::sync::Arc;

use serenity::all::{ChannelId, Context, CreateMessage, MessageBuilder};

use r2d2wm_core::Message;

pub async fn send_to_discord(msg: &Message, ctx: Arc<Context>) {
    let channel: ChannelId = ChannelId::new(msg.channel_id.get());
    let message: CreateMessage = build_discord_message(msg);

    match channel.send_message(&ctx.http, message.clone()).await {
        Ok(msg) => {
            tracing::info!("Message sent on schedule: {:?}", &msg.id);
            tracing::trace!("{:?}", &msg);
        }
        Err(e) => {
            tracing::error!("Failed to send message: {:?}: {:?}", &msg.id, e);
        }
    }
}

fn build_discord_message(msg: &Message) -> CreateMessage {
    let mut msg_builder: MessageBuilder = MessageBuilder::new();
    msg_builder.push(&msg.content);
    CreateMessage::new().content(msg_builder.build())
}
