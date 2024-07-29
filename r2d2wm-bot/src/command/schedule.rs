use crate::command::DiscordCommand;
use crate::scheduler::persistence;
use crate::util::ToDiscordString;
use anyhow::Result;
use async_trait::async_trait;
use r2d2wm_core::{Message, Task, TaskMode, TaskState};
use regex::Regex;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateInputText, CreateInteractionResponse, CreateInteractionResponseMessage, InputTextStyle,
};
use serenity::utils::CreateQuickModal;
use std::num::NonZeroU64;

pub struct ListSchedules;

#[async_trait]
impl DiscordCommand for ListSchedules {
    fn register(&self) -> CreateCommand {
        CreateCommand::new("schedule_ls").description("List active schedule")
    }

    async fn run(&self, ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        let schedules = persistence::read_tasks_for_guild().await?;
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

pub struct AddSchedule;

#[async_trait]
impl DiscordCommand for AddSchedule {
    fn register(&self) -> CreateCommand {
        CreateCommand::new("schedule_add").description("Add new schedule")
    }

    async fn run(&self, ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        let re = Regex::new(r"^(((\d+,)+\d+|(\d+([/\-])\d+)|\d+|\*) ?){5}$")?;
        let (mut name, mut cron, mut msg) = ("", "", "");

        let modal = CreateQuickModal::new("New scheduled task")
            .field(
                CreateInputText::new(InputTextStyle::Short, "Name", "name")
                    .placeholder("scheduled_name")
                    .value(name)
                    .required(true),
            )
            .field(
                CreateInputText::new(InputTextStyle::Short, "Cron", "cron")
                    .placeholder("* * * * *")
                    .value(cron)
                    .required(true),
            )
            .field(
                CreateInputText::new(InputTextStyle::Paragraph, "Message", "msg")
                    .value(msg)
                    .required(true),
            );

        let resp = interaction.quick_modal(ctx, modal).await?.unwrap();
        (name, cron, msg) = (&resp.inputs[0], &resp.inputs[1], &resp.inputs[2]);

        if !re.is_match(cron) {
            resp.interaction
                .create_response(
                    ctx,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new()
                            .content("❌ Invalid Cron expression, cancelling...")
                            .ephemeral(true),
                    ),
                )
                .await?;

            return Ok(());
        };

        persistence::create_task(Task {
            id: None,
            name: name.to_string(),
            cron_expr: cron.to_string(),
            mode: TaskMode::Repeat,
            state: TaskState::Enabled,
            guild_id: NonZeroU64::new(1).unwrap(),
            message: Message {
                id: None,
                content: msg.to_string(),
                guild_id: NonZeroU64::new(1).unwrap(),
                channel_id: NonZeroU64::new(1).unwrap(),
            },
        })
        .await?;

        resp.interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("✅ Message saved!")
                        .ephemeral(true),
                ),
            )
            .await?;

        Ok(())
    }
}

pub struct RemoveSchedule;

#[async_trait]
impl DiscordCommand for RemoveSchedule {
    fn register(&self) -> CreateCommand {
        CreateCommand::new("schedule_rm")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::String,
                    "name",
                    "The name of the scheduled message you wish to delete",
                )
                .required(true),
            )
            .description("Remove schedule")
    }

    async fn run(&self, ctx: &Context, interaction: &CommandInteraction) -> Result<()> {
        interaction
            .create_response(
                ctx,
                CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content("Not implemented yet")
                        .ephemeral(true),
                ),
            )
            .await?;

        Ok(())
    }
}
