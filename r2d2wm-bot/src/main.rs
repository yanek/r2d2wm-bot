#![warn(clippy::pedantic)]

mod bot;
mod config;
mod error;
mod handler;
mod log;

pub use self::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    log::init()?;
    bot::run().await
}
