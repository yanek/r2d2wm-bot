#![warn(clippy::pedantic)]

mod bot;
mod environment;
mod error;
mod handler;

pub use self::error::{Error, Result};

#[tokio::main]
async fn main() -> Result<()> {
    environment::init()?;
    bot::run().await?;
    Ok(())
}
