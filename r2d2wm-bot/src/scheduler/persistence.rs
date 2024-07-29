use std::env;

use r2d2wm_core::Task;

pub async fn read_tasks_for_guild() -> anyhow::Result<Vec<Task>> {
    let uri = format!("{}/tasks/guilds/1", env::var("API_URI")?);
    let client = reqwest::Client::new();
    let tasks: Vec<Task> = client.get(uri).send().await?.json().await?;
    Ok(tasks)
}

pub async fn create_task(task: Task) -> anyhow::Result<Task> {
    let uri = format!("{}/tasks", env::var("API_URI")?);
    let client = reqwest::Client::new();
    let task = client.post(uri).json(&task).send().await?.json().await?;
    Ok(task)
}
