use crate::data::{connect_db, GetForGuild, RowMapping};
use crate::{Error, Result};
use itertools::Itertools;
use r2d2wm_core::{Message, Task};
use std::num::NonZeroU64;
use uuid::Uuid;

impl GetForGuild for Task {
    type Target = Task;

    fn get_many_for_guild(guild_id: NonZeroU64) -> Result<Vec<Task>> {
        let conn = connect_db()?;
        let query = r#"
        SELECT * FROM tasks t 
        INNER JOIN messages m 
        WHERE m.id = t.message_id AND t.guild_id = ?"#;

        let mut stmt = conn
            .prepare(query)
            .map_err(|e| Error::Internal(e.to_string()))?;

        let rows = stmt
            .query_map([guild_id.get()], Self::map_row)
            .map_err(|e| Error::BadQuery(e.to_string()))?;

        let out: Vec<Task> = rows
            .into_iter()
            .try_collect()
            .map_err(|e| Error::BadQuery(e.to_string()))?;

        if out.is_empty() {
            return Err(Error::NotFound("Response is empty".to_string()));
        }

        Ok(out)
    }
}

impl RowMapping for Task {
    type Target = Task;

    fn map_row(row: &rusqlite::Row) -> std::result::Result<Task, rusqlite::Error> {
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
                guild_id: row.get("guild_id")?,
                channel_id: row.get("channel_id")?,
            },
        })
    }
}
