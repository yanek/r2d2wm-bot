use crate::command::ping::Ping;
use crate::command::schedule::ListSchedules;
use crate::{Error, Result};
use anyhow::bail;
use serenity::all::{Command, CommandInteraction, Context, CreateCommand, Http};
use serenity::async_trait;
use std::collections::HashMap;
use std::sync::OnceLock;

pub mod ping;
mod schedule;

#[async_trait]
pub trait DiscordCommand {
    fn register(&self) -> CreateCommand;
    async fn run(&self, ctx: &Context, interaction: &CommandInteraction) -> anyhow::Result<()>;
}

type BoxedCommand = Box<dyn DiscordCommand + Send + Sync>;
fn available_commands() -> &'static HashMap<String, BoxedCommand> {
    static AVAIL_CMDS: OnceLock<HashMap<String, BoxedCommand>> = OnceLock::new();
    AVAIL_CMDS.get_or_init(|| {
        let mut m: HashMap<String, BoxedCommand> = HashMap::new();
        m.insert("ping".to_string(), Box::new(Ping));
        m.insert("schedule_ls".to_string(), Box::new(ListSchedules));
        m
    })
}

pub async fn register_all(http: &Http) -> Vec<Result<Command>> {
    let mut results = Vec::new();

    for command in available_commands().values() {
        results.push(
            Command::create_global_command(http, command.register())
                .await
                .map_err(Error::CreateCommand),
        );
    }

    results
}

pub async fn run(ctx: &Context, interaction: &CommandInteraction) -> anyhow::Result<()> {
    let name = &interaction.data.name;
    tracing::debug!(
        "Received command: (name: {:?}, from: {:?}, on: {:?})",
        name,
        &interaction.user.name,
        &interaction.guild_id.unwrap_or_default().name(&ctx.cache),
    );

    if let Some(cmd) =
        available_commands()
            .iter()
            .find_map(|(n, c)| if *name == *n { Some(c) } else { None })
    {
        return cmd.run(ctx, interaction).await;
    };

    bail!("Command not found: {}", name.clone());
}
