use anyhow::{Context, Result};
use itertools::Itertools;
use r2d2wm_core::{Message, Task};
use rusqlite::Connection;
use std::num::NonZeroU64;
use std::{env, path::Path};
use uuid::Uuid;

pub struct TaskAccessObject {
    connection: Connection,
}

impl TaskAccessObject {
    pub fn new() -> Result<Self> {
        let connection = connect_db()?;
        Ok(Self { connection })
    }

    pub fn get_in_guild(&self, guild_id: NonZeroU64) -> Result<Vec<Task>> {
        let query = "SELECT * FROM tasks t INNER JOIN messages m WHERE m.id = t.message_id AND t.guild_id = ?";
        let mut stmt = self.connection.prepare(query)?;
        let rows = stmt.query_map([guild_id.get()], |row| {
            Ok(Task {
                id: Uuid::parse_str(&row.get::<&str, String>("id")?).unwrap(),
                name: row.get("name")?,
                cron_expr: row.get("cron")?,
                state: row.get("state")?,
                mode: row.get("repeat_mode")?,
                guild_id: row.get("guild_id")?,
                message: Message {
                    id: Uuid::parse_str(&row.get::<&str, String>("message_id")?).unwrap(),
                    content: row.get("content")?,
                    guild_id: NonZeroU64::new(row.get::<&str, u64>("guild_id")?).unwrap(),
                    channel_id: NonZeroU64::new(row.get::<&str, u64>("channel_id")?.into())
                        .unwrap(),
                },
            })
        })?;
        Ok(rows.into_iter().try_collect()?)
    }
}

fn connect_db() -> Result<Connection> {
    let db_path = Path::new(&env::var("DATA_PATH").unwrap()).join("db.sqlite");
    Connection::open(db_path).context("Failed to connect to the database")
}
