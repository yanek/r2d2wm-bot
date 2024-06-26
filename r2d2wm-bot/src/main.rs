#![warn(clippy::pedantic)]

mod bot;
mod config;
mod error;
mod log;

use crate::config::Config;
pub use crate::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config: Config = Config::load()?;
    log::init(&config);
    bot::start(&config.app.discord_token, config.schedules).await
}
