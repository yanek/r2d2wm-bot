use rocket::serde::json::Json;
use rocket::{launch, routes, Build, Rocket};

use r2d2wm_core::Task;

pub use crate::error::{Error, Result};
use crate::routes::*;

mod data;
mod error;
mod routes;

type ManyTasks = Json<Vec<Task>>;

#[launch]
fn rocket() -> Rocket<Build> {
    let _ = dotenvy::dotenv();
    rocket::build().mount("/api/tasks/", routes![guild_tasks])
}
