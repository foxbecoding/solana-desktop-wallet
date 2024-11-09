use rusqlite::{params, Connection};
use crate::database::errors::DatabaseError;

#[derive(Debug)]
pub struct Wallet {
    pub id: Option<i32>,
    pub name: String,
    pub seed: String,
    pub public_key: String,
    pub is_passphrase_protected: bool,
}


// Function to insert a new wallet into the wallets table
pub fn insert_wallet(conn: &Connection, wallet: &Wallet) -> Result<usize, DatabaseError> {
    conn.execute(
        "INSERT INTO wallets (name, seed, public_key, is_passphrase_protected) VALUES (?1, ?2, ?3, ?4)",
        params![
            &wallet.name,
            &wallet.seed,
            &wallet.public_key,
            &wallet.is_passphrase_protected
        ],
    ).map_err(DatabaseError::from)
}

// Function to retrieve all wallets from the wallets table
pub fn get_wallets(conn: &Connection) -> Result<Vec<Wallet>, DatabaseError> {
    let query = "SELECT id, name, seed, public_key, is_passphrase_protected FROM wallets";
    let mut stmt = conn.prepare(query)?;
    let wallet_iter = stmt.query_map([], |row| {
        Ok(Wallet {
            id: row.get(0)?,
            name: row.get(1)?,
            seed: row.get(2)?,
            public_key: row.get(3)?,
            is_passphrase_protected: row.get(4)?,
        })
    })?;

    let mut wallets = Vec::new();
    for wallet_result in wallet_iter {
        wallets.push(wallet_result?);
    }
    Ok(wallets)
}