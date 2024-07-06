#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod bot;
mod command;
mod config;
mod error;
mod log;
mod scheduler;

pub use crate::error::{Error, Result};
use std::env;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenvy::dotenv();
    log::init();
    let tz_str = env::var("TIMEZONE")?;
    let timezone = chrono_tz::Tz::from_str(&tz_str)?;
    let token = env::var("DISCORD_TOKEN")?;
    bot::start(&token, timezone).await
}
