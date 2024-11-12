pub mod account;
pub mod errors;

use rusqlite::{Connection, Result};
use crate::database::errors::DatabaseError;


pub fn database_connection() -> Result<Connection, DatabaseError> {
    let conn = Connection::open("resources/database/database.db")?;
    Ok(conn)
}