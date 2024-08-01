use anyhow::Result;
use chrono_tz::Tz;
use colored::Colorize;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::GuildId;
use poise::Framework;
use r2d2wm_core::Environment;

use crate::commands;
use crate::scheduler::persistence::read_tasks_for_guild;
use crate::scheduler::Scheduler;

pub struct Data {
    timezone: Tz,
}

pub type PoiseContext<'a> = poise::Context<'a, Data, anyhow::Error>;

pub async fn start(token: &str, timezone: Tz) -> Result<()> {
    let intents: serenity::GatewayIntents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = Framework::<Data, anyhow::Error>::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::ping(),
                commands::r2ls(),
                commands::r2add(),
                commands::r2rm(),
            ],
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
                Ok(Data { timezone })
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
