use crate::Result;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct Config {
    pub discord_token: String,
    pub logging_level: String,
    pub schedules: Option<Vec<ScheduledMessage>>,
}

impl Config {
    pub fn from_file() -> Result<Self> {
        let usr: Option<String> = env::var("R2_CONFIG_PATH").ok();
        let def: String = "config".to_string();
        let path: PathBuf = Path::new(&usr.unwrap_or(def)).join("config.toml");

        tracing::debug!("Reading config from {:?}", path);
        let data: String = std::fs::read_to_string(path)?;
        let conf: Config = toml::from_str(&data)?;
        tracing::trace!("{:?}", conf);

        Ok(conf)
    }
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
pub struct ScheduledMessage {
    pub name: String,
    pub cron: String,
    pub recipients: Option<Vec<String>>,
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{assert_tokens, Token};

    #[test]
    fn test_ser_config() {
        let conf = Config {
            discord_token: "token".to_string(),
            logging_level: "info".to_string(),
            schedules: None,
        };

        assert_tokens(
            &conf,
            &[
                Token::Struct {
                    name: "Config",
                    len: 3,
                },
                Token::Str("discord_token"),
                Token::Str("token"),
                Token::Str("logging_level"),
                Token::Str("info"),
                Token::Str("schedules"),
                Token::None,
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
        let conf: Config = toml::from_str(toml).unwrap();
        assert_eq!(
            conf,
            Config {
                discord_token: "token".to_string(),
                logging_level: "info".to_string(),
                schedules: None,
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
