use rusqlite::{params, Connection};
use slint::SharedString;
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

fn get_account_pubkey_display(pubkey: String) -> SharedString {
    let input_string = pubkey;

    // Get the first 5 characters
    let first_part = &input_string[0..5];
    // Get the last 4 characters
    let last_part = &input_string[input_string.len() - 4..];

    // Combine with "..."
    let combined_string = format!("{}...{}", first_part, last_part);

    SharedString::from(combined_string)
}