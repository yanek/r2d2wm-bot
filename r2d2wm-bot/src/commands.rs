use std::num::NonZeroU64;

use crate::{
    bot::PoiseContext,
    scheduler::persistence::{create_task, read_tasks_for_guild},
    util::ToDiscordString,
};
use anyhow::{Context, Result};
use r2d2wm_core::{Message, Task, TaskMode, TaskState};

#[poise::command(slash_command, ephemeral)]
pub async fn ping(ctx: PoiseContext<'_>) -> Result<()> {
    let response = "Pong! 🏓".to_string();
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, ephemeral)]
pub async fn r2ls(ctx: PoiseContext<'_>) -> Result<()> {
    let Some(guild_id) = ctx.guild_id() else {
        ctx.say("This command must be run in a server.").await?;
        return Ok(());
    };
    let tasks = read_tasks_for_guild(guild_id).await?;
    let response = tasks
        .iter()
        .map(ToDiscordString::to_discord_string)
        .collect::<Vec<String>>()
        .join("\n");
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, ephemeral)]
pub async fn r2add(
    ctx: PoiseContext<'_>,
    #[rename = "cron"] cron_expr: String,
    #[rename = "message"] message_content: String,
) -> Result<()> {
    let emsg = ("Invalid guild ID", "Invalid channel ID");
    let guild_id = NonZeroU64::new(ctx.guild_id().context(emsg.0)?.get()).context(emsg.0)?;
    let channel_id = NonZeroU64::new(ctx.channel_id().get()).context(emsg.1)?;

    let message = Message {
        id: None,
        content: message_content,
        channel_id,
        guild_id,
    };

    let task = Task {
        id: None,
        cron_expr,
        mode: TaskMode::Repeat,
        state: TaskState::Enabled,
        guild_id,
        message,
    };

    create_task(task).await?;

    let res = "👌 Message added to the schedule.";
    ctx.say(res.to_string()).await?;
    Ok(())
}

#[poise::command(slash_command, ephemeral)]
pub async fn r2rm(ctx: PoiseContext<'_>, task_id: u64) -> Result<()> {
    let response = format!("(fake) Removed task: {task_id}");
    ctx.say(response).await?;
    Ok(())
}
