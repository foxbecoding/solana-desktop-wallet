use rusqlite::{params, Connection};
use crate::database::errors::DatabaseError;

#[derive(Debug)]
pub struct Wallet {
    pub id: Option<i32>,
    pub name: String,
    pub seed: String,
}

// Function to create the wallets table if it doesn't exist
pub fn create_wallets_table(conn: &Connection) -> Result<(), DatabaseError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS wallets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            seed TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

// Function to insert a new wallet into the wallets table
pub fn insert_wallet(conn: &Connection, wallet: &Wallet) -> Result<usize, DatabaseError> {
    conn.execute(
        "INSERT INTO wallets (name, seed) VALUES (?1, ?2)",
        params![&wallet.name, &wallet.seed],
    ).map_err(DatabaseError::from)
}

// Function to retrieve all wallets from the wallets table
pub fn get_wallets(conn: &Connection) -> Result<Vec<Wallet>, DatabaseError> {
    let mut stmt = conn.prepare("SELECT id, name, seed FROM wallets")?;
    let wallet_iter = stmt.query_map([], |row| {
        Ok(Wallet {
            id: row.get(0)?,
            name: row.get(1)?,
            seed: row.get(2)?,
        })
    })?;

    let mut wallets = Vec::new();
    for wallet_result in wallet_iter {
        wallets.push(wallet_result?);
    }
    Ok(wallets)
}