#![warn(clippy::pedantic)]

mod bot;
mod config;
mod error;
mod log;

use crate::config::Config;
pub use crate::error::{Error, Result};
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    let config: Config = Config::load()?;
    log::init(&config);
    let timezone = chrono_tz::Tz::from_str(&config.app.timezone)?;
    bot::start(&config.app.discord_token, timezone, config.schedules).await
}
