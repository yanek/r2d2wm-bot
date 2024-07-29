use std::sync::Arc;

use serenity::all::{ChannelId, Context, CreateMessage, MessageBuilder};

use r2d2wm_core::Task;

use crate::util::ToDiscordString;

pub async fn send_to_discord(task: &Task, ctx: Arc<Context>) {
    let channel: ChannelId = ChannelId::new(task.message.channel_id.get());
    let message: CreateMessage = build_discord_message(task);

    match channel.send_message(&ctx.http, message.clone()).await {
        Ok(msg) => {
            tracing::info!("Message sent on schedule: {:?}", &msg.id);
            tracing::trace!("{:?}", &msg);
        }
        Err(e) => {
            tracing::error!("Failed to send message: {:?}: {:?}", &task.id, e);
        }
    }
}

fn build_discord_message(task: &Task) -> CreateMessage {
    let mut msg_builder: MessageBuilder = MessageBuilder::new();
    msg_builder.push(&task.to_discord_string());
    CreateMessage::new().content(msg_builder.build())
}

impl ToDiscordString for Task {
    fn to_discord_string(&self) -> String {
        format!(
            "## {}\n```sh\n# Cron:\n{}\n\n# Message:\n{}```",
            self.name, self.cron_expr, self.message.content
        )
    }
}
