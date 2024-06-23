use crate::Result;
use serde::Deserialize;
use std::env;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub discord_token: String,
    pub logging_level: String,
}

impl Config {
    pub fn from_file() -> Result<Self> {
        let usr: Option<String> = env::var("R2_CONFIG_PATH").ok();
        let def: String = "config".to_string();
        let conf_path: PathBuf = Path::new(&usr.unwrap_or(def)).join("config.toml");

        tracing::trace!("Reading config from {:?}", conf_path);
        let conf_data = std::fs::read_to_string(conf_path)?;
        let conf = toml::from_str(&conf_data)?;
        tracing::trace!("{:?}", conf);

        Ok(conf)
    }
}
