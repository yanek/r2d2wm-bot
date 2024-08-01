use std::num::NonZeroU64;

use anyhow::{Context, Result};
use async_std::channel;
use chrono_tz::Tz;
use colored::Colorize;
use poise::serenity_prelude::model::guild;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::GuildId;
use poise::Framework;
use r2d2wm_core::{Environment, Message, Task, TaskMode, TaskState};

use crate::scheduler::persistence::{create_task, read_tasks_for_guild};
use crate::scheduler::Scheduler;
use crate::util::ToDiscordString;

pub struct Data {
    scheduled_messages: Vec<Task>,
    timezone: Tz,
}

pub type PoiseContext<'a> = poise::Context<'a, Data, anyhow::Error>;

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
    #[rename = "cron"]cron_expr: String,
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

pub async fn start(token: &str, timezone: Tz) -> Result<()> {
    let intents: serenity::GatewayIntents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = Framework::<Data, anyhow::Error>::builder()
        .options(poise::FrameworkOptions {
            commands: vec![ping(), r2ls(), r2add(), r2rm()],
            event_handler: |ctx, event, _framework, data| Box::pin(event_handler(ctx, event, data)),
            pre_command: |ctx| {
                Box::pin(async move {
                    pre_command(ctx);
                })
            },
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                let commands = &framework.options().commands;
                match Environment::get() {
                    Environment::Production => {
                        poise::builtins::register_globally(ctx, commands).await?;
                    }
                    Environment::Development => {
                        let guild = GuildId::new(705_869_087_292_653_708);
                        poise::builtins::register_in_guild(ctx, commands, guild).await?;
                    }
                }
                Ok(Data {
                    scheduled_messages: vec![],
                    timezone,
                })
            })
        })
        .build();

    serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await?
        .start()
        .await?;

    Ok(())
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    data: &Data,
) -> Result<()> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            tracing::info!("Logged in as '{}'.", data_about_bot.user.name.blue());
        }
        serenity::FullEvent::CacheReady { guilds, .. } => {
            let sched = Scheduler::new(ctx.clone(), data.timezone).await?;

            for guild_id in guilds {
                let guild = ctx.http.get_guild(*guild_id).await?;
                tracing::info!("## Starting jobs in {}...", guild.name.italic());
                let tasks = read_tasks_for_guild(*guild_id).await?;
                let mut counter = 0;
                for task in &tasks {
                    sched.push(task.clone()).await?;
                    counter += 1;
                }
                tracing::info!("## Added {counter} job(s).");
            }
        }
        _ => {}
    };
    Ok(())
}

/// Executed before every command.
fn pre_command(ctx: PoiseContext<'_>) {
    let guild_name = match ctx.guild() {
        Some(g) => g.name.clone(),
        None => "None".to_string(),
    };

    tracing::info!(
        "Received /{} command from '{}' in '{}'.",
        ctx.command().qualified_name,
        ctx.author().name.italic(),
        guild_name.italic(),
    );
}
