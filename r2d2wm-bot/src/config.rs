use crate::Result;
use serde::{Deserialize, Serialize};
use std::env;
use std::ops::Deref;
use std::path::{Path, PathBuf};

const ENV_CONFIG_PATH: &str = "R2D2WM_CONFIG_PATH";

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct AppSettings {
    pub discord_token: String,
    pub logging_level: String,
    pub timezone: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub struct ScheduledMessage {
    pub name: String,
    pub cron: String,
    pub recipients: Option<Vec<String>>,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ScheduledMessageList {
    schedules: Vec<ScheduledMessage>,
}

impl Deref for ScheduledMessageList {
    type Target = Vec<ScheduledMessage>;

    fn deref(&self) -> &Self::Target {
        &self.schedules
    }
}

pub struct Config {
    pub app: AppSettings,
    pub schedules: ScheduledMessageList,
}

impl Config {
    pub fn load() -> Result<Self> {
        Ok(Config {
            app: Self::get_app_settings_from_file()?,
            schedules: Self::get_schedules_from_file()?,
        })
    }

    fn get_app_settings_from_file() -> Result<AppSettings> {
        let path = Self::construct_path_to("settings.toml");
        let data: String = std::fs::read_to_string(path)?;
        let config: AppSettings = toml::from_str(&data)?;
        Ok(config)
    }

    pub fn get_schedules_from_file() -> Result<ScheduledMessageList> {
        let path = Self::construct_path_to("schedules.toml");
        let data: String = std::fs::read_to_string(path)?;
        let schedules: ScheduledMessageList = toml::from_str(&data)?;
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
    use serde_test::{assert_tokens, Token};

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
    fn test_ser_config() {
        assert_tokens(
            &app_settings(),
            &[
                Token::Struct {
                    name: "AppSettings",
                    len: 3,
                },
                Token::Str("discord_token"),
                Token::Str("token"),
                Token::Str("logging_level"),
                Token::Str("info"),
                Token::Str("timezone"),
                Token::Str("Europe/Paris"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_de_config() {
        let toml = r#"
            discord_token = "token"
            logging_level = "info"
            timezone = "Europe/Paris"
        "#;
        let conf: AppSettings = toml::from_str(toml).unwrap();
        assert_eq!(conf, app_settings(),);
    }

    #[test]
    fn test_ser_schedule() {
        assert_tokens(
            &scheduled_message(),
            &[
                Token::Struct {
                    name: "ScheduledMessage",
                    len: 4,
                },
                Token::Str("name"),
                Token::Str("test"),
                Token::Str("cron"),
                Token::Str("0 0 * * * * *"),
                Token::Str("recipients"),
                Token::None,
                Token::Str("message"),
                Token::Str("test"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_de_schedule() {
        let toml = r#"
            name = "test"
            cron = "0 0 * * * * *"
            message = "test"
        "#;
        let sched: ScheduledMessage = toml::from_str(toml).unwrap();
        assert_eq!(sched, scheduled_message());
    }
}
