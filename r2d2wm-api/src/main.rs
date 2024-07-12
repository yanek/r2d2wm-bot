use rocket::{launch, routes, Build, Rocket};

pub use crate::error::{Error, Result};
use crate::routes::*;

mod data;
mod error;
mod routes;

#[launch]
fn rocket() -> Rocket<Build> {
    let _ = dotenvy::dotenv();
    rocket::build().mount("/api/", routes![guild_tasks, task])
}
