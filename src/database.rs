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
    fn test_in_memory_database_connection() {}
}
