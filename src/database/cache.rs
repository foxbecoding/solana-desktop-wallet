use crate::database::errors::DatabaseError;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

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
    conn: Arc<Mutex<Connection>>,
}

impl Cache {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Cache { conn }
    }

    fn set(&self, key: CacheKey, value: &CacheValue) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let value = serde_json::to_string(value).unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO cache (key, value) VALUES (?1, ?2)",
            params![key.key(), value],
        )?;
        Ok(())
    }

    fn get(&self, key: CacheKey) -> Result<Option<String>, DatabaseError> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM cache WHERE key = ?1")?;
        let mut rows = stmt.query(params![key.key()])?;

        if let Some(row) = rows.next()? {
            let value: String = row.get(0)?;
            let cache_value: CacheValue = serde_json::from_str(&value).unwrap();
            Ok(Some(cache_value.value))
        } else {
            Ok(None)
        }
    }

    fn remove(&self, key: CacheKey) -> Result<(), DatabaseError> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM cache WHERE key = ?1", params![key.key()])?;
        Ok(())
    }

    pub fn set_selected_account(&self, value: &CacheValue) -> Result<(), DatabaseError> {
        self.set(CacheKey::SelectedAccount, value)
    }

    pub fn get_selected_account(&self, key: CacheKey) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::database_connection;

    fn setup_test_db() -> Arc<Mutex<Connection>> {
        let conn = Arc::new(Mutex::new(database_connection().unwrap()));
        let conn_clone_binding = conn.clone();
        let conn_clone = conn_clone_binding.lock().unwrap();
        conn_clone
            .execute(
                "CREATE TABLE cache (key TEXT PRIMARY KEY, value TEXT NOT NULL)",
                [],
            )
            .unwrap();
        conn
    }

    #[test]
    fn test_set_and_get_cache_value() {
        let conn = setup_test_db();
        let cache = Cache::new(conn);

        let key = CacheKey::SelectedAccount;
        let value = CacheValue {
            value: "TestAccount".to_string(),
        };

        // Insert value into cache
        cache.set(&key, &value).unwrap();

        // Retrieve value from cache
        let fetched_value = cache.get(&key).unwrap();

        assert!(fetched_value.is_some());
        assert_eq!(fetched_value.unwrap(), "TestAccount");
    }

    #[test]
    fn test_get_nonexistent_cache_value() {
        let conn = setup_test_db();
        let cache = Cache::new(conn);

        let key = CacheKey::SelectedAccount;

        // Try fetching a nonexistent value
        let fetched_value = cache.get(&key).unwrap();

        assert!(fetched_value.is_none());
    }

    #[test]
    fn test_remove_cache_value() {
        let conn = setup_test_db();
        let cache = Cache::new(conn);

        let key = CacheKey::SelectedAccount;
        let value = CacheValue {
            value: "ToBeRemoved".to_string(),
        };

        // Insert value into cache
        cache.set(&key, &value).unwrap();

        // Remove the value
        cache.remove(&key).unwrap();

        // Try fetching the removed value
        let fetched_value = cache.get(&key).unwrap();

        assert!(fetched_value.is_none());
    }
}
