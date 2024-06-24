#![warn(clippy::pedantic)]

mod bot;
mod config;
mod error;
mod handler;
mod log;

pub use self::error::Result;
use crate::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;
    log::init(&config)?;
    bot::run(&config).await
}
