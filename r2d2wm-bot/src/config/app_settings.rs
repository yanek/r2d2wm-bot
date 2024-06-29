use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub struct AppSettings {
    pub discord_token: String,
    pub logging_level: String,
    pub timezone: String,
}
