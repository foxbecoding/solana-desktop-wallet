use rusqlite::{params, Connection};
use crate::database::errors::DatabaseError;

#[derive(Debug)]
pub struct Account {
    pub id: Option<i32>,
    pub name: String,
    pub seed: String,
    pub pubkey: String,
    pub is_passphrase_protected: bool,
}


// Function to insert a new account into the accounts table
pub fn insert_account(conn: &Connection, account: &Account) -> Result<usize, DatabaseError> {
    conn.execute(
        "INSERT INTO accounts (name, seed, pubkey, is_passphrase_protected) VALUES (?1, ?2, ?3, ?4)",
        params![
            &account.name,
            &account.seed,
            &account.pubkey,
            &account.is_passphrase_protected
        ],
    ).map_err(DatabaseError::from)
}

// Function to retrieve all accounts from the accounts table
pub fn get_accounts(conn: &Connection) -> Result<Vec<Account>, DatabaseError> {
    let query = "SELECT id, name, seed, pubkey, is_passphrase_protected FROM accounts";
    let mut stmt = conn.prepare(query)?;
    let account_iter = stmt.query_map([], |row| {
        Ok(Account {
            id: row.get(0)?,
            name: row.get(1)?,
            seed: row.get(2)?,
            pubkey: row.get(3)?,
            is_passphrase_protected: row.get(4)?,
        })
    })?;

    let mut accounts = Vec::new();
    for account_result in account_iter {
        accounts.push(account_result?);
    }
    Ok(accounts)
}