use anyhow::Result;
use poise::serenity_prelude::GuildId;
use r2d2wm_core::Task;
use std::env;
use std::num::NonZeroU64;

pub async fn read_tasks_for_guild(guild_id: GuildId) -> Result<Vec<Task>> {
    let uri = format!("{}/tasks/guilds/{}", env::var("API_URI")?, guild_id);
    let client = reqwest::Client::new();
    let tasks: Vec<Task> = client.get(uri).send().await?.json().await?;
    Ok(tasks)
}

pub async fn create_task(task: Task) -> Result<Task> {
    let uri = format!("{}/tasks", env::var("API_URI")?);
    let client = reqwest::Client::new();
    dbg!(serde_json::to_string(&task));
    let task = client.post(uri).json(&task).send().await?.json().await?;
    dbg!(&task);
    Ok(task)
}

pub async fn delete_task(task_id: NonZeroU64) -> Result<()> {
    let uri = format!("{}/tasks/{}", env::var("API_URI")?, task_id);
    let client = reqwest::Client::new();
    client.delete(&uri).send().await?;
    Ok(())
}
