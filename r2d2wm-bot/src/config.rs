use crate::Result;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::{Path, PathBuf};

const ENV_CONFIG_PATH: &str = "R2D2WM_CONFIG_PATH";

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct AppSettings {
    pub discord_token: String,
    pub logging_level: String,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ScheduledMessage {
    pub name: String,
    pub cron: String,
    pub recipients: Option<Vec<String>>,
    pub message: String,
}

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
        let path = Self::construct_path_to("settings.toml");

        tracing::debug!("Reading config from {:?}", path);
        let data: String = std::fs::read_to_string(path)?;
        let config: AppSettings = toml::from_str(&data)?;
        tracing::trace!("{:?}", config);

        Ok(config)
    }

    pub fn get_schedules_from_file() -> Result<Vec<ScheduledMessage>> {
        let path = Self::construct_path_to("schedules.toml");

        tracing::debug!("Reading schedules from {:?}", path);
        let data: String = std::fs::read_to_string(path)?;
        let schedules: Vec<ScheduledMessage> = toml::from_str(&data)?;
        tracing::debug!("Found {} schedules", schedules.len());
        tracing::trace!("{:?}", schedules);

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
        let conf = AppSettings {
            discord_token: "token".to_string(),
            logging_level: "info".to_string(),
        };

        assert_tokens(
            &conf,
            &[
                Token::Struct {
                    name: "AppSettings",
                    len: 2,
                },
                Token::Str("discord_token"),
                Token::Str("token"),
                Token::Str("logging_level"),
                Token::Str("info"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_de_config() {
        let toml = r#"
            discord_token = "token"
            logging_level = "info"
        "#;
        let conf: AppSettings = toml::from_str(toml).unwrap();
        assert_eq!(
            conf,
            AppSettings {
                discord_token: "token".to_string(),
                logging_level: "info".to_string(),
            }
        );
    }

    #[test]
    fn test_ser_schedule() {
        let sched = ScheduledMessage {
            name: "test".to_string(),
            cron: "0 0 * * * * *".to_string(),
            recipients: None,
            message: "test".to_string(),
        };

        assert_tokens(
            &sched,
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
        assert_eq!(
            sched,
            ScheduledMessage {
                name: "test".to_string(),
                cron: "0 0 * * * * *".to_string(),
                recipients: None,
                message: "test".to_string(),
            }
        );
    }
}
