use super::{connect_db, Create, Delete, ReadById, ReadManyInGuild, RowMapping};
use anyhow::{bail, Context, Result};
use itertools::Itertools;
use r2d2wm_core::{Message, MessageId, Task, TaskId};
use std::num::NonZeroU64;

impl ReadManyInGuild for Task {
    type EntryType = Task;

    fn read_many_in_guild(guild_id: NonZeroU64) -> Result<Vec<Task>> {
        let conn = connect_db()?;

        let query = r#"
        SELECT * FROM tasks t 
        INNER JOIN messages m 
        WHERE m.id = t.message_id AND t.guild_id = ?"#;

        let mut stmt = conn.prepare(query)?;

        let rows: Vec<Task> = stmt
            .query_map([guild_id.get()], Self::map_row)?
            .try_collect()?;

        if rows.is_empty() {
            bail!("Response is empty");
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

        let mut stmt = conn.prepare(query)?;
        let rows: Vec<Task> = stmt.query_map([id], Self::map_row)?.try_collect()?;
        rows.into_iter().next().context("Task not found")
    }
}

impl Create for Task {
    type EntryType = Task;

    fn create(task: &Task) -> Result<Task> {
        let mut conn = connect_db()?;
        let transac = conn.transaction()?;

        let query = r#"
        INSERT INTO messages 
        (content, guild_id, channel_id) 
        VALUES (?, ?, ?)
        "#;

        transac
            .execute(
                query,
                (
                    &task.message.content,
                    &task.message.guild_id,
                    &task.message.channel_id,
                ),
            )
            .context("Cannot create message")?;

        let message_id =
            NonZeroU64::new(transac.last_insert_rowid() as u64).context("Cannot map ID")?;

        let query = r#"
        INSERT INTO tasks
        (cron, repeat_mode, state, guild_id, message_id)
        VALUES (?, ?, ?, ?, ?)
        "#;

        transac.execute(
            query,
            (
                &task.cron_expr,
                &task.mode,
                &task.state,
                &task.guild_id,
                &message_id,
            ),
        )?;

        let task_id =
            NonZeroU64::new(transac.last_insert_rowid() as u64).context("Cannot map ID")?;

        transac.commit()?;

        let mut ret = task.clone();
        ret.id = Some(TaskId::new(task_id));
        ret.message.id = Some(MessageId::new(message_id));

        Ok(ret)
    }
}

impl Delete for Task {
    type EntryType = Task;

    fn delete(id: NonZeroU64) -> Result<()> {
        let conn = connect_db()?;

        let query = r#"
            DELETE FROM tasks
            WHERE id = ?
        "#;

        conn.prepare(query)?.execute([id])?;

        Ok(())
    }
}

impl RowMapping for Task {
    type EntryType = Task;

    fn map_row(row: &rusqlite::Row) -> std::result::Result<Task, rusqlite::Error> {
        Ok(Task {
            id: row.get("id")?,
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
