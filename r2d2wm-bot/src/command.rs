use crate::{Error, Result};
use serenity::all::{Command, Http};

pub mod ping;

pub async fn register_commands(http: &Http) -> Vec<Result<Command>> {
    let commands = vec![ping::register()];
    let mut results = Vec::new();

    for command in commands {
        results.push(
            Command::create_global_command(http, command)
                .await
                .map_err(Error::CreateCommand),
        );
    }

    results
}
