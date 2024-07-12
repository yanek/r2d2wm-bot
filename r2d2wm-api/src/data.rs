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

pub trait GetForGuild {
    type Target;
    fn get_many_for_guild(guild_id: NonZeroU64) -> Result<Vec<Self::Target>>;
}

pub trait RowMapping {
    type Target;
    fn map_row(row: &rusqlite::Row) -> std::result::Result<Self::Target, rusqlite::Error>;
}
