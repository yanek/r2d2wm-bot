use std::{string, sync::Arc};

use poise::serenity_prelude as serenity;

use r2d2wm_core::Task;

use crate::util::ToDiscordString;

pub async fn send_to_discord(task: Task, ctx: Arc<serenity::Context>) {
    let channel = serenity::ChannelId::new(task.message.channel_id.get());
    let message: serenity::CreateMessage = build_discord_message(&task);

    match channel.send_message(&ctx.http, message.clone()).await {
        Ok(msg) => {
            tracing::info!("Message sent on schedule: {:?}", &task.id);
            tracing::trace!("{:?}", (&msg.id, &msg.channel_id, &msg.guild_id));
        }
        Err(e) => {
            tracing::error!("Failed to send message: {:?}: {:?}", &task.id, e);
        }
    }
}

fn build_discord_message(task: &Task) -> serenity::CreateMessage {
    let mut msg_builder = serenity::MessageBuilder::new();
    msg_builder.push(&task.message.content);
    serenity::CreateMessage::new().content(msg_builder.build())
}

impl ToDiscordString for Task {
    fn to_discord_string(&self) -> String {
        let id = self
            .id
            .as_ref()
            .map_or_else(|| "INVALID_ID".to_string(), string::ToString::to_string);

        format!(
            "```sh\n# Task ID:\n{}\n\n# Cron:\n{}\n\n# Message:\n{}```",
            id, self.cron_expr, self.message.content
        )
    }
}
