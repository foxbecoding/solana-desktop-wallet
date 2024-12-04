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

fn create_test_tables(conn: &Connection) {}

fn create_test_accounts_table(conn: &Connection) {
    conn.execute(
        "CREATE TABLE accounts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                seed TEXT NOT NULL,
                pubkey TEXT NOT NULL,
                passphrase TEXT NOT NULL,
                balance INTEGER
            )",
        [],
    )
    .unwrap();
}
