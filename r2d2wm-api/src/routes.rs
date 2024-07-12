use std::num::NonZeroU64;

use rocket::get;
use rocket::serde::json::Json;

use r2d2wm_core::Task;

use crate::data::GetForGuild;
use crate::{Error, ManyTasks};

#[get("/<guild_id>")]
pub fn guild_tasks(guild_id: Option<NonZeroU64>) -> crate::Result<ManyTasks> {
    let guild_id = guild_id.ok_or(Error::BadQuery("Invalid guild ID".to_string()))?;
    let msgs = Task::get_many_for_guild(guild_id)?;
    Ok(Json(msgs))
}
