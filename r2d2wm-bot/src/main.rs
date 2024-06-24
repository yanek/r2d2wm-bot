#![warn(clippy::pedantic)]

mod bot;
mod config;
mod error;
mod handler;
mod log;

use crate::config::Config;
pub use crate::error::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;
    log::init(&config);
    bot::run(&config).await
}
