use crate::config::Config;

pub fn init(config: &Config) {
    let level = &config.app.logging_level;
    let filter = format!("info,r2d2wm_bot={level}");
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_line_number(true)
        .init();
    tracing::debug!("Logging initialized with level: {level:?}");
}
