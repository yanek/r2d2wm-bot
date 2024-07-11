use crate::data::TaskAccessObject;
use r2d2wm_common::Task;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, launch, routes, Build, Rocket};
use std::num::NonZeroU64;

mod data;

#[get("/")]
fn index() -> Result<Json<Vec<Task>>, Custom<String>> {
    let q =
        TaskAccessObject::new().map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
    let msgs = q
        .get_in_guild(NonZeroU64::new(1).unwrap())
        .map_err(|e| Custom(Status::InternalServerError, e.to_string()))?;
    Ok(Json(msgs))
}

#[launch]
fn rocket() -> Rocket<Build> {
    let _ = dotenvy::dotenv();
    rocket::build().mount("/", routes![index])
}
