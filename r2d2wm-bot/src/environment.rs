use crate::Result;
use std::env;
use std::path::Path;
use tracing_subscriber::EnvFilter;

pub fn init() -> Result<()> {
    config_env()?;
    config_tracing();
    Ok(())
}

fn config_env() -> Result<()> {
    let cfgpath_base = env::var("R2D2WM_CONFIG_PATH").unwrap_or(".".to_string());
    let cfgpath = Path::new(&cfgpath_base);
    let envfile = cfgpath.join(".env");
    dotenv::from_path(envfile)?;
    Ok(())
}

fn config_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_line_number(true)
        .init();
    tracing::trace!("tracing configured");
}
