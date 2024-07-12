use itertools::Itertools;
use r2d2wm_core::{Message, Task};
use std::num::NonZeroU64;

use super::{connect_db, ReadById, ReadManyInGuild, RowMapping};
use crate::{Error, Result};

impl ReadManyInGuild for Task {
    type EntryType = Task;

    fn read_many_in_guild(guild_id: NonZeroU64) -> Result<Vec<Task>> {
        let conn = connect_db()?;
        let query = r#"
        SELECT * FROM tasks t 
        INNER JOIN messages m 
        WHERE m.id = t.message_id AND t.guild_id = ?"#;

        let mut stmt = conn
            .prepare(query)
            .map_err(|e| Error::Internal(e.to_string()))?;

        let rows: Vec<Task> = stmt
            .query_map([guild_id.get()], Self::map_row)
            .map_err(|e| Error::BadQuery(e.to_string()))?
            .try_collect()
            .map_err(|e| Error::BadQuery(e.to_string()))?;

        if rows.is_empty() {
            return Err(Error::NotFound("Response is empty".to_string()));
        }

        Ok(rows)
    }
}

impl ReadById for Task {
    type EntryType = Task;

    fn read_by_id(id: NonZeroU64) -> Result<Task> {
        let conn = connect_db()?;
        let query = r#"
        SELECT * FROM tasks t
        INNER JOIN messages m
        WHERE m.id = t.message_id AND t.id = ?"#;

        let mut stmt = conn
            .prepare(query)
            .map_err(|e| Error::Internal(e.to_string()))?;

        let rows: Vec<Task> = stmt
            .query_map([id], Self::map_row)
            .map_err(|e| Error::BadQuery(e.to_string()))?
            .try_collect()
            .map_err(|e| Error::BadQuery(e.to_string()))?;

        rows.into_iter()
            .next()
            .ok_or(Error::NotFound("Task not found".to_string()))
    }
}

impl RowMapping for Task {
    type EntryType = Task;

    fn map_row(row: &rusqlite::Row) -> std::result::Result<Task, rusqlite::Error> {
        Ok(Task {
            id: row.get("id")?,
            name: row.get("name")?,
            cron_expr: row.get("cron")?,
            state: row.get("state")?,
            mode: row.get("repeat_mode")?,
            guild_id: row.get("guild_id")?,
            message: Message {
                id: row.get("message_id")?,
                content: row.get("content")?,
                guild_id: row.get("guild_id")?,
                channel_id: row.get("channel_id")?,
            },
        })
    }
}
