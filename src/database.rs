pub mod account;
pub mod errors;
use rusqlite::{Connection, Result};

pub fn database_connection() -> Result<Connection, errors::DatabaseError> {
    let conn = Connection::open("resources/database/database.db")?;
    Ok(conn)
}