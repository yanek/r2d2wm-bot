use crate::config::{AppSettings, Config};
use crate::Result;

pub fn init(config: &Config) -> Result<()> {
    let level = &config.app.logging_level;
    let filter = format!("info,r2d2wm_bot={level}");
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_line_number(true)
        .init();
    tracing::trace!("Logging initialized");
    Ok(())
}
