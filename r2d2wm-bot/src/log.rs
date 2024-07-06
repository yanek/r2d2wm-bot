use std::env;

pub fn init() {
    let filter = env::var("RUST_LOG").unwrap_or("trace".to_string());
    tracing_subscriber::fmt()
        .with_env_filter(&filter)
        .without_time()
        .with_line_number(true)
        .init();
    tracing::debug!("Logging initialized with level: {filter:?}");
}
