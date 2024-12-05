use rusqlite::{Connection, Result};
pub mod account;
pub mod cache;
pub mod errors;

use crate::database::errors::DatabaseError;

pub fn database_connection() -> Result<Connection, DatabaseError> {
    let conn = if cfg!(test) {
        Connection::open_in_memory()?
    } else {
        Connection::open("resources/database/database.db")?
    };
    Ok(conn)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_memory_database_connection() {
        // Ensure the connection is opened in memory during testing
        let conn = database_connection().expect("Failed to create in-memory database connection");

        // Test creating a table in the in-memory database
        conn.execute(
            "CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
            [],
        )
        .expect("Failed to create table in in-memory database");

        // Insert a value into the test table
        conn.execute("INSERT INTO test (name) VALUES ('test_name')", [])
            .expect("Failed to insert into in-memory database");

        // Query the value
        let mut stmt = conn
            .prepare("SELECT name FROM test WHERE id = 1")
            .expect("Failed to prepare statement");
        let result: String = stmt
            .query_row([], |row| row.get(0))
            .expect("Failed to query in-memory database");

        // Check if the value matches
        assert_eq!(result, "test_name");
    }

    #[test]
    fn test_file_database_connection() {}
}
