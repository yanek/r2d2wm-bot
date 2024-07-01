use serenity::all::{CreateCommand, ResolvedOption};

pub fn register() -> CreateCommand {
    CreateCommand::new("ping").description("Check bot connectivity")
}

pub fn run(_options: &[ResolvedOption]) -> String {
    "Pong!".to_string()
}
