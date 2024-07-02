use crate::command::DiscordCommand;
use crate::Result;
use async_trait::async_trait;
use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};

pub struct Ping;

#[async_trait]
impl DiscordCommand for Ping {
    fn register(&self) -> CreateCommand {
        CreateCommand::new("ping").description("Check bot connectivity")
    }

    async fn run(&self, ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Pong!")
                        .ephemeral(true),
                ),
            )
            .await?;

        Ok(())
    }
}
