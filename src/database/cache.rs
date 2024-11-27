use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use crate::database::{database_connection, errors::DatabaseError};

pub enum CacheKey {
    SelectedAccount,
    SelectedView,
}

impl CacheKey {
    pub fn key(&self) -> String {
        match self {
            CacheKey::SelectedAccount => "selected_account".to_string(),
            CacheKey::SelectedView => "selected_view".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CacheValue {
    pub value: String,
}

pub struct Cache {
    pub conn: Connection,
}

impl Cache {
    pub fn new() -> Result<Self, DatabaseError> {
        let conn = database_connection()?;
        Ok(Cache { conn })
    }

    pub fn insert(&self, key: &CacheKey, value: &CacheValue) -> Result<(), DatabaseError> {
        let value = serde_json::to_string(value).unwrap();
        self.conn.execute(
            "INSERT OR REPLACE INTO cache (key, value) VALUES (?1, ?2)",
            params![key.key(), value],
        )?;
        Ok(())
    }

    pub fn get(&self, key: &str) -> Result<Option<CacheValue>, DatabaseError> {
        let mut stmt = self.conn.prepare("SELECT value FROM cache WHERE key = ?1")?;
        let mut rows = stmt.query(params![key])?;

        if let Some(row) = rows.next()? {
            let value: String = row.get(0)?;
            let cache_value: CacheValue = serde_json::from_str(&value).unwrap();
            Ok(Some(cache_value))
        } else {
            Ok(None)
        }
    }

    pub fn remove(&self, key: &str) -> Result<(), DatabaseError> {
        self.conn.execute("DELETE FROM cache WHERE key = ?1", params![key])?;
        Ok(())
    }
}