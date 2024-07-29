use crate::{
    data::{Create, Delete, ReadById, ReadManyInGuild},
    AppError,
};
use axum::{extract::Path, http::StatusCode, Json};
use r2d2wm_core::Task;
use std::num::NonZeroU64;

type ManyTasksResponse = Json<Vec<Task>>;
type OneTaskResponse = Json<Task>;

pub async fn get_guilds_tasks(guild_id: Path<NonZeroU64>) -> Result<ManyTasksResponse, AppError> {
    tracing::info!("GET tasks where guild = {}", *guild_id);
    let msgs = Task::read_many_in_guild(*guild_id)?;
    Ok(Json(msgs))
}

pub async fn get_task_by_id(task_id: Path<NonZeroU64>) -> Result<OneTaskResponse, AppError> {
    tracing::info!("GET task where id = {}", *task_id);
    let t = Task::read_by_id(*task_id)?;
    Ok(Json(t))
}

pub async fn post_task(input: Json<Task>) -> Result<OneTaskResponse, AppError> {
    tracing::info!("POST {:?}", input);
    let t = Task::create(&input)?;
    Ok(Json(t))
}

pub async fn delete_task(task_id: Path<NonZeroU64>) -> Result<StatusCode, AppError> {
    tracing::info!("DELETE task where id = {}", *task_id);
    Task::delete(*task_id)?;
    Ok(StatusCode::NO_CONTENT)
}
