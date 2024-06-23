use crate::Result;
use serde::Deserialize;
use std::env;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub discord_token: String,
    pub logging_level: String,
    pub schedules: Vec<ScheduledMessage>,
}

impl Config {
    pub fn from_file() -> Result<Self> {
        let usr: Option<String> = env::var("R2_CONFIG_PATH").ok();
        let def: String = "config".to_string();
        let conf_path: PathBuf = Path::new(&usr.unwrap_or(def)).join("config.toml");

        tracing::trace!("Reading config from {:?}", conf_path);
        let conf_data: String = std::fs::read_to_string(conf_path)?;
        let conf: Config = toml::from_str(&conf_data)?;
        tracing::trace!("{:?}", conf);

        Ok(conf)
    }
}

#[derive(Deserialize, Debug)]
pub struct ScheduledMessage {
    pub name: String,
    pub cron: String,
    pub recipients: Option<Vec<String>>,
    pub message: String,
}
