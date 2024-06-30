use crate::config::Config;

const DEFAULT_LOG_LEVEL: &str = "warn";
const PACKAGE_NAME: &str = "r2d2wm_bot";

pub fn init(config: &Config) {
    let level = &config.app.logging_level;
    let filter = format!("{DEFAULT_LOG_LEVEL},{PACKAGE_NAME}={level}");
    tracing_subscriber::fmt()
        .with_env_filter(&filter)
        .with_line_number(true)
        .init();
    tracing::debug!("Logging initialized with level: {filter:?}");
}
