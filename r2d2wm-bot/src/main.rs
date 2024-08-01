#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod bot;
mod scheduler;
mod util;
mod commands;

use colored::Colorize;
use r2d2wm_core::Environment;
use std::env;
use std::str::FromStr;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    tracing_subscriber::fmt()
        .with_thread_names(false)
        .with_env_filter(EnvFilter::from_default_env())
        .without_time()
        .with_line_number(false)
        .with_target(false)
        .init();

    tracing::info!("Starting r2d2wm-bot...");
    tracing::info!(
        "Started in '{}' environment.",
        Environment::get().to_string().blue()
    );

    let tz_str = env::var("TIMEZONE")?;
    let timezone = chrono_tz::Tz::from_str(&tz_str)?;
    tracing::info!("Selected timezone is '{}'.", timezone.to_string().blue());

    let token = env::var("DISCORD_TOKEN")?;

    tracing::info!("Initializing...");
    bot::start(&token, timezone).await
}
