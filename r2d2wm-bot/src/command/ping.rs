use crate::Result;
use serenity::all::{
    CommandInteraction, Context, CreateCommand, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("Check bot connectivity")
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
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
