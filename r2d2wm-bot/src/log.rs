use std::env;

const DEFAULT_LOG_LEVEL: &str = "warn";
const PACKAGE_NAME: &str = "r2d2wm_bot";

pub fn init() {
    let filter = env::var("RUST_LOG").unwrap_or("trace".to_string());
    tracing_subscriber::fmt()
        .with_env_filter(&filter)
        .with_line_number(true)
        .init();
    tracing::debug!("Logging initialized with level: {filter:?}");
}
