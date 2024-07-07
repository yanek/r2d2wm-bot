use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};

use crate::command::DiscordCommand;
use crate::scheduler::persistence;
use crate::util::ToDiscordString;

pub struct ListSchedules;

#[async_trait]
impl DiscordCommand for ListSchedules {
    fn register(&self) -> CreateCommand {
        CreateCommand::new("schedule_ls").description("List active schedule")
    }

    async fn run(&self, ctx: &Context, interaction: &CommandInteraction) -> anyhow::Result<()> {
        let schedules = persistence::get_all_messages().await?;
        let mut content_parts: Vec<String> = Vec::new();

        for sched in &schedules {
            content_parts.push(sched.to_discord_string());
        }

        interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(content_parts.join("\n"))
                        .ephemeral(true),
                ),
            )
            .await?;

        Ok(())
    }
}
