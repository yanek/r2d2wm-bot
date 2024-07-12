use crate::data::GetForGuild;
use r2d2wm_core::Task;
use rocket::serde::json::Json;
use rocket::{get, launch, routes, Build, Rocket};
use std::num::NonZeroU64;

mod data;
mod error;

pub use crate::error::{Error, Result};

type ManyTasks = Json<Vec<Task>>;

#[get("/<guild_id>")]
fn index(guild_id: Option<NonZeroU64>) -> Result<ManyTasks> {
    let guild_id = guild_id.ok_or(Error::BadQuery("Invalid guild ID".to_string()))?;
    let msgs = Task::get_many_for_guild(guild_id)?;
    Ok(Json(msgs))
}

#[launch]
fn rocket() -> Rocket<Build> {
    let _ = dotenvy::dotenv();
    rocket::build().mount("/", routes![index])
}
