use crate::{
    data::{Create, Delete, ReadById, ReadManyInGuild},
    AppError,
};
use axum::{extract::Path, http::StatusCode, Json};
use r2d2wm_core::Task;
use std::num::NonZeroU64;

type ManyTasksResponse = Json<Vec<Task>>;
type OneTaskResponse = Json<Task>;

pub async fn get_guild_tasks(guild_id: Path<NonZeroU64>) -> Result<ManyTasksResponse, AppError> {
    dbg!(&guild_id);
    let msgs = Task::read_many_in_guild(*guild_id)?;
    Ok(Json(msgs))
}

pub async fn get_task_by_id(task_id: Path<NonZeroU64>) -> Result<OneTaskResponse, AppError> {
    let t = Task::read_by_id(*task_id)?;
    Ok(Json(t))
}

pub async fn post_task(input: Json<Task>) -> Result<OneTaskResponse, AppError> {
    let t = Task::create(&input)?;
    Ok(Json(t))
}

pub async fn delete_task(task_id: Path<NonZeroU64>) -> Result<StatusCode, AppError> {
    Task::delete(*task_id)?;
    Ok(StatusCode::NO_CONTENT)
}
