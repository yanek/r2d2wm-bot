use std::num::NonZeroU64;
use std::sync::Arc;

use serde::Deserialize;
use serenity::all::{ChannelId, Context, CreateMessage, MessageBuilder, RoleId, UserId};

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type", content = "id", rename_all = "lowercase")]
pub enum MentionTarget {
    Role(NonZeroU64),
    User(NonZeroU64),
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct ScheduledMessage {
    pub name: String,
    pub cron: String,
    pub channel_id: NonZeroU64,
    pub mentions: Option<Vec<MentionTarget>>,
    pub message: String,
}

impl ScheduledMessage {
    pub async fn send_to_discord(&self, ctx: Arc<Context>) {
        let channel: ChannelId = ChannelId::new(self.channel_id.get());
        let message: CreateMessage = self.build_discord_message();

        match channel.send_message(&ctx.http, message.clone()).await {
            Ok(msg) => {
                tracing::info!("Message sent on schedule: {:?}", &msg.id);
                tracing::trace!("{:?}", &msg);
            }
            Err(e) => {
                tracing::error!("Failed to send message: {:?}: {:?}", &self.name, e);
            }
        }
    }

    fn build_discord_message(&self) -> CreateMessage {
        let mut msg_builder: MessageBuilder = MessageBuilder::new();
        msg_builder.push(&self.message);

        if let Some(mentions) = &self.mentions {
            msg_builder.push_line("");

            for (i, target) in mentions.iter().enumerate() {
                match target {
                    MentionTarget::Role(id) => msg_builder.mention(&RoleId::new(id.get())),
                    MentionTarget::User(id) => msg_builder.mention(&UserId::new(id.get())),
                };
                if i < mentions.len() - 1 {
                    msg_builder.push(" ");
                }
            }
        }

        CreateMessage::new().content(msg_builder.build())
    }
}
