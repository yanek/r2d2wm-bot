use std::env;
use std::path::{Path, PathBuf};

pub use scheduled_message::ScheduledMessage;

use crate::Result;

mod app_settings;
mod scheduled_message;

const ENV_DATA_PATH: &str = "DATA_PATH";
const SCHEDULE_FILENAME: &str = "schedule.json";
const DEFAULT_CONFIG_DIRECTORY: &str = "config";

#[derive(Debug)]
pub struct Config {
    pub schedules: Vec<ScheduledMessage>,
}

impl Config {
    pub fn load() -> Result<Self> {
        Ok(Config {
            schedules: Self::get_schedules_from_file()?,
        })
    }

    fn get_schedules_from_file() -> Result<Vec<ScheduledMessage>> {
        let path = Self::construct_path_to(SCHEDULE_FILENAME);
        if !path.exists() {
            tracing::info!("Creating missing schedule file...");
            std::fs::write(&path, "[]")?;
        }
        let data: String = std::fs::read_to_string(path)?;
        let schedules: Vec<ScheduledMessage> = serde_json::from_str(&data).unwrap_or_else(|e| {
            tracing::error!("{e}: {e:?}");
            Vec::new()
        });
        Ok(schedules)
    }

    fn construct_path_to(filename: &str) -> PathBuf {
        let usr: Option<String> = env::var(ENV_DATA_PATH).ok();
        let def: String = DEFAULT_CONFIG_DIRECTORY.to_string();
        Path::new(&usr.unwrap_or(def)).join(filename)
    }
}
