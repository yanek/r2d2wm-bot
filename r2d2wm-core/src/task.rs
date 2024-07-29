use std::num::NonZeroU64;

use rusqlite::types::{FromSql, FromSqlResult, ToSqlOutput, Value, ValueRef};
use rusqlite::ToSql;
use serde::{Deserialize, Serialize};

use crate::Message;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: Option<NonZeroU64>,
    pub name: String,
    pub cron_expr: String,
    pub mode: TaskMode,
    pub state: TaskState,
    pub guild_id: NonZeroU64,
    pub message: Message,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum TaskState {
    Disabled = 0,
    Enabled = 1,
}

impl FromSql for TaskState {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let val_str = value.as_str()?;
        match val_str {
            "disabled" => Ok(TaskState::Disabled),
            "enabled" => Ok(TaskState::Enabled),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

impl ToSql for TaskState {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Text(
            match &self {
                TaskState::Disabled => "disabled",
                TaskState::Enabled => "enabled",
            }
            .to_string(),
        )))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "snake_case")]
#[repr(u8)]
pub enum TaskMode {
    Repeat = 0,
    OneShot = 1,
}

impl FromSql for TaskMode {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let val_str = value.as_str()?;
        match val_str {
            "repeat" => Ok(TaskMode::Repeat),
            "one_shot" => Ok(TaskMode::OneShot),
            _ => Err(rusqlite::types::FromSqlError::InvalidType),
        }
    }
}

impl ToSql for TaskMode {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::Owned(Value::Text(
            match &self {
                TaskMode::Repeat => "repeat",
                TaskMode::OneShot => "one_shot",
            }
            .to_string(),
        )))
    }
}
