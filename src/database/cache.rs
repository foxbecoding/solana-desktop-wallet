use crate::database::{database_connection, errors::DatabaseError};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

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

    pub fn get(&self, key: &CacheKey) -> Result<Option<CacheValue>, DatabaseError> {
        let mut stmt = self
            .conn
            .prepare("SELECT value FROM cache WHERE key = ?1")?;
        let mut rows = stmt.query(params![key.key()])?;

        if let Some(row) = rows.next()? {
            let value: String = row.get(0)?;
            let cache_value: CacheValue = serde_json::from_str(&value).unwrap();
            Ok(Some(cache_value))
        } else {
            Ok(None)
        }
    }

    pub fn remove(&self, key: &CacheKey) -> Result<(), DatabaseError> {
        self.conn
            .execute("DELETE FROM cache WHERE key = ?1", params![key.key()])?;
        Ok(())
    }
}

pub fn fetch_cache_value(key: &CacheKey) -> Result<Option<String>, DatabaseError> {
    let cache = Cache::new()?;
    if let Some(value) = cache.get(key)? {
        Ok(Some(value.value))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_db() -> Connection {
        let conn = database_connection().unwrap();
        conn.execute(
            "CREATE TABLE cache (key TEXT PRIMARY KEY, value TEXT NOT NULL)",
            [],
        )
        .unwrap();
        conn
    }

    #[test]
    fn test_insert_and_get_cache_value() {}

    #[test]
    fn test_get_nonexistent_cache_value() {}

    #[test]
    fn test_remove_cache_value() {}

    #[test]
    fn test_fetch_cache_value() {}

    #[test]
    fn test_fetch_cache_value_nonexistent() {}
}
