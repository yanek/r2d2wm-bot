use crate::command::DiscordCommand;
use crate::scheduler::persistence;
use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};

pub struct ListSchedules;

#[async_trait]
impl DiscordCommand for ListSchedules {
    fn register(&self) -> CreateCommand {
        CreateCommand::new("schedule_ls").description("List active schedule")
    }

    async fn run(&self, ctx: &Context, interaction: &CommandInteraction) -> crate::Result<()> {
        let schedules = persistence::get_all_messages()?;
        let mut embeds = Vec::new();

        for sched in &schedules {
            embeds.push(
                CreateEmbed::new()
                    .title(&sched.name)
                    .description(format!("```rust\n{:#?}```", &sched)),
            );
        }

        interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .add_embeds(embeds)
                        .ephemeral(true),
                ),
            )
            .await?;

        Ok(())
    }
}
