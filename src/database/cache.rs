use rusqlite::{Connection};
use serde::{Deserialize, Serialize};
use crate::database::{database_connection, errors::DatabaseError};

#[derive(Serialize, Deserialize, Debug)]
struct CacheValue {
    value: String,
}

struct Cache {
    conn: Connection,
}

impl Cache {
    fn new() -> Result<Self, DatabaseError> {
        let conn = database_connection()?;
        Ok(Cache { conn })
    }

    fn insert() -> Result<(), DatabaseError> {Ok(())}

    fn get() -> Result<(), DatabaseError> {Ok(())}
}