use std::env;
use std::path::{Path, PathBuf};

pub use app_settings::AppSettings;
pub use scheduled_message::ScheduledMessage;

use crate::Result;

mod app_settings;
mod scheduled_message;

const ENV_CONFIG_PATH: &str = "R2D2WM_CONFIG_PATH";
const APP_SETTINGS_FILENAME: &str = "app_config.json";
const SCHEDULE_FILENAME: &str = "schedule.json";
const DEFAULT_CONFIG_DIRECTORY: &str = "config";

#[derive(Debug)]
pub struct Config {
    pub app: AppSettings,
    pub schedules: Vec<ScheduledMessage>,
}

impl Config {
    pub fn load() -> Result<Self> {
        Ok(Config {
            app: Self::get_app_settings_from_file()?,
            schedules: Self::get_schedules_from_file()?,
        })
    }

    fn get_app_settings_from_file() -> Result<AppSettings> {
        let path = Self::construct_path_to(APP_SETTINGS_FILENAME);
        let data: String = std::fs::read_to_string(path)?;
        let config: AppSettings = serde_json::from_str(&data)?;
        Ok(config)
    }

    fn get_schedules_from_file() -> Result<Vec<ScheduledMessage>> {
        let path = Self::construct_path_to(SCHEDULE_FILENAME);
        let data: String = std::fs::read_to_string(path)?;
        let schedules: Vec<ScheduledMessage> = serde_json::from_str(&data)?;
        Ok(schedules)
    }

    fn construct_path_to(filename: &str) -> PathBuf {
        let usr: Option<String> = env::var(ENV_CONFIG_PATH).ok();
        let def: String = DEFAULT_CONFIG_DIRECTORY.to_string();
        Path::new(&usr.unwrap_or(def)).join(filename)
    }
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU64;

    use serde_json::json;

    use super::*;

    fn app_settings() -> AppSettings {
        AppSettings {
            discord_token: "token".to_string(),
            logging_level: "info".to_string(),
            timezone: "Europe/Paris".to_string(),
        }
    }

    fn scheduled_message() -> ScheduledMessage {
        ScheduledMessage {
            name: "test".to_string(),
            cron: "0 0 * * * * *".to_string(),
            channel_id: NonZeroU64::new(1).unwrap(),
            mentions: None,
            message: "test".to_string(),
        }
    }

    #[test]
    fn test_construct_path_to() {
        let path = Config::construct_path_to("test.json");
        assert_eq!(path, PathBuf::from("config/test.json"));
    }

    #[test]
    fn test_construct_path_to_with_env() {
        env::set_var(ENV_CONFIG_PATH, "test");
        let path = Config::construct_path_to("test.json");
        assert_eq!(path, PathBuf::from("test/test.json"));
    }

    #[test]
    fn test_de_config() {
        let json = json!({
            "discord_token": "token",
            "logging_level": "info",
            "timezone": "Europe/Paris"
        })
        .to_string();
        let conf: AppSettings = serde_json::from_str(&json).expect("Failed to parse JSON");
        assert_eq!(conf, app_settings(),);
    }

    #[test]
    fn test_de_schedule() {
        let json = json!({
            "name": "test",
            "cron": "0 0 * * * * *",
            "channel_id": 1,
            "message": "test"
        })
        .to_string();

        let sched: ScheduledMessage = serde_json::from_str(&json).expect("Failed to parse JSON");
        assert_eq!(sched, scheduled_message());
    }
}
