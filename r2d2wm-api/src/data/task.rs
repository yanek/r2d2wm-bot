use crate::data::{connect_db, GetForGuild, RowMapping};
use itertools::Itertools;
use r2d2wm_core::{Message, Task};
use std::num::NonZeroU64;
use uuid::Uuid;

impl GetForGuild for Task {
    type Target = Task;

    fn get_many_for_guild(guild_id: NonZeroU64) -> anyhow::Result<Vec<Task>> {
        let conn = connect_db()?;
        let query = "SELECT * FROM tasks t INNER JOIN messages m WHERE m.id = t.message_id AND t.guild_id = ?";
        let mut stmt = conn.prepare(query)?;
        let rows = stmt.query_map([guild_id.get()], Self::map_row)?;
        Ok(rows.into_iter().try_collect()?)
    }
}

impl RowMapping for Task {
    type Target = Task;

    fn map_row(row: &rusqlite::Row) -> anyhow::Result<Task, rusqlite::Error> {
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
