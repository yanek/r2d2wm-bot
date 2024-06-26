use crate::Result;
use serde::Deserialize;
use serenity::all::ChannelId;
use std::env;
use std::path::{Path, PathBuf};

const ENV_CONFIG_PATH: &str = "R2D2WM_CONFIG_PATH";

#[derive(Deserialize, Debug, PartialEq)]
pub struct AppSettings {
    pub discord_token: String,
    pub logging_level: String,
    pub timezone: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
pub struct ScheduledMessage {
    pub name: String,
    pub cron: String,
    pub channel_id: ChannelId,
    pub recipients: Option<Vec<String>>,
    pub message: String,
}

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
        let path = Self::construct_path_to("app_config.json");
        let data: String = std::fs::read_to_string(path)?;
        let config: AppSettings = serde_json::from_str(&data)?;
        Ok(config)
    }

    pub fn get_schedules_from_file() -> Result<Vec<ScheduledMessage>> {
        let path = Self::construct_path_to("schedule.json");
        let data: String = std::fs::read_to_string(path)?;
        let schedules: Vec<ScheduledMessage> = serde_json::from_str(&data)?;
        Ok(schedules)
    }

    fn construct_path_to(filename: &str) -> PathBuf {
        let usr: Option<String> = env::var(ENV_CONFIG_PATH).ok();
        let def: String = "config".to_string();
        Path::new(&usr.unwrap_or(def)).join(filename)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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
            channel_id: ChannelId::new(1),
            recipients: None,
            message: "test".to_string(),
        }
    }

    #[test]
    fn test_construct_path_to() {
        let path = Config::construct_path_to("test.toml");
        assert_eq!(path, PathBuf::from("config/test.toml"));
    }

    #[test]
    fn test_construct_path_to_with_env() {
        env::set_var(ENV_CONFIG_PATH, "test");
        let path = Config::construct_path_to("test.toml");
        assert_eq!(path, PathBuf::from("test/test.toml"));
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
