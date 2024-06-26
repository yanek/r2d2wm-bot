#![warn(clippy::pedantic)]

mod bot;
mod config;
mod error;
mod log;
mod scheduler;

use crate::bot::Bot;
use crate::config::Config;
pub use crate::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;

    log::init(&config);

    let bot = Bot::new(&config.app.discord_token);
    bot.start(config.schedules).await
}
