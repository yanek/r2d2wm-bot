use std::num::{NonZero, NonZeroU64};

use rocket::get;
use rocket::serde::json::Json;

use r2d2wm_core::Task;

use crate::data::{ReadById, ReadManyInGuild};
use crate::{Error, Result};

type ManyTasksResponse = Json<Vec<Task>>;
type OneTaskResponse = Json<Task>;

#[get("/guild/<guild_id>/tasks")]
pub fn guild_tasks(guild_id: Option<NonZeroU64>) -> Result<ManyTasksResponse> {
    let guild_id = guild_id.ok_or(Error::BadQuery("Invalid guild ID".to_string()))?;
    let msgs = Task::read_many_in_guild(guild_id)?;
    Ok(Json(msgs))
}

#[get("/tasks/<task_id>")]
pub fn task(task_id: Option<NonZeroU64>) -> Result<OneTaskResponse> {
    let task_id = task_id.ok_or(Error::BadQuery("Invalid task ID".to_string()))?;
    let t = Task::read_by_id(task_id)?;
    Ok(Json(t))
}
