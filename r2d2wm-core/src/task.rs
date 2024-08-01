use std::fmt;
use std::num::NonZeroU64;

use rusqlite::types::{FromSql, FromSqlResult, ToSqlOutput, Value, ValueRef};
use rusqlite::ToSql;
use serde::{Deserialize, Serialize};

use crate::Message;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct TaskId(NonZeroU64);

impl TaskId {
    pub fn new(id: NonZeroU64) -> Self {
        Self(id)
    }

    pub fn get(&self) -> NonZeroU64 {
        self.0
    }
}

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromSql for TaskId {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        let id = value.as_i64()?;
        Ok(TaskId(NonZeroU64::new(id as u64).unwrap()))
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: Option<TaskId>,
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
