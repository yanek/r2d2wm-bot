use std::env;

pub fn init() {
    let filter = env::var("RUST_LOG").unwrap_or("trace".to_string());

    tracing::debug!("Logging initialized with level: {filter:?}");
}
