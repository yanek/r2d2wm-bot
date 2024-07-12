mod task;

use crate::{Error, Result};
use rusqlite::Connection;
use std::num::NonZeroU64;
use std::{env, path::Path};

fn connect_db() -> Result<Connection> {
    let db_path = Path::new(&env::var("DATA_PATH").unwrap()).join("db.sqlite");
    Connection::open(db_path)
        .map_err(|_| Error::ServiceUnavailable("Database unreachable".to_string()))
}

pub trait ReadManyInGuild {
    type EntryType;
    fn read_many_in_guild(guild_id: NonZeroU64) -> Result<Vec<Self::EntryType>>;
}

pub trait ReadById {
    type EntryType;
    fn read_by_id(id: NonZeroU64) -> Result<Self::EntryType>;
}

pub trait Create {
    type EntryType;
    fn create(entry: &Self::EntryType) -> Result<()>;
}

pub trait Update {
    type EntryType;
    fn update(id: NonZeroU64, new_entry: &Self::EntryType) -> Result<()>;
}

pub trait Delete {
    type EntryType;
    fn delete(id: NonZeroU64) -> Result<()>;
}

pub trait RowMapping {
    type EntryType;
    fn map_row(row: &rusqlite::Row) -> std::result::Result<Self::EntryType, rusqlite::Error>;
}
