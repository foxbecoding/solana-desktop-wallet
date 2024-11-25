use rusqlite::{params, Connection};
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

    fn insert(&self, key: &str, value: &CacheValue) -> Result<(), DatabaseError> {
        let value = serde_json::to_string(value).unwrap();
        self.conn.execute(
            "INSERT OR REPLACE INTO cache (key, value) VALUES (?1, ?2)",
            params![key, value],
        )?;
        Ok(())
    }

    fn get() -> Result<(), DatabaseError> {Ok(())}

    fn remove() -> Result<(), DatabaseError> {Ok(())}
}