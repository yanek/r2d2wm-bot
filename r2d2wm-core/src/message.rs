use core::fmt;
use rusqlite::types::FromSql;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU64;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct MessageId(NonZeroU64);

impl MessageId {
    pub fn new(id: NonZeroU64) -> Self {
        Self(id)
    }

    pub fn get(&self) -> NonZeroU64 {
        self.0
    }
}

impl fmt::Display for MessageId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromSql for MessageId {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        let id = value.as_i64()?;
        Ok(MessageId(NonZeroU64::new(id as u64).unwrap()))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Message {
    pub id: Option<MessageId>,
    pub content: String,
    pub guild_id: NonZeroU64,
    pub channel_id: NonZeroU64,
}
