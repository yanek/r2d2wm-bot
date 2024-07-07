#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod bot;
mod command;
mod scheduler;
mod util;

use std::env;
use std::str::FromStr;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    tracing_subscriber::fmt()
        .pretty()
        .with_thread_names(false)
        .with_env_filter(EnvFilter::from_default_env())
        .without_time()
        .with_line_number(true)
        .init();

    let tz_str = env::var("TIMEZONE")?;
    let timezone = chrono_tz::Tz::from_str(&tz_str)?;
    let token = env::var("DISCORD_TOKEN")?;
    bot::start(&token, timezone).await
}
