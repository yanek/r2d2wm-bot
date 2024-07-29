use std::env;

use r2d2wm_core::Task;

pub async fn get_all_messages() -> anyhow::Result<Vec<Task>> {
    let uri = format!("{}/tasks/guilds/1", env::var("API_URI")?);
    let body = reqwest::get(uri).await?.text().await?;
    let tasks: Vec<Task> = serde_json::from_str(&body).unwrap_or_else(|e| {
        tracing::error!("{e}: {e:?}");
        Vec::new()
    });
    Ok(tasks)
}
