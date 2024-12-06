use crate::database::errors::DatabaseError;
use bip39::{Error as MnemonicError, Mnemonic};
use rusqlite::{params, Connection};
use serde::de::StdError;
use slint::SharedString;
use solana_sdk::native_token::lamports_to_sol;
use solana_sdk::pubkey::{ParsePubkeyError, Pubkey};
use solana_sdk::signature::{keypair, Keypair};
use solana_sdk::signer::Signer;
use std::{
    error::Error,
    str::FromStr,
    sync::{Arc, Mutex},
};

#[derive(Debug, Clone)]
pub struct Account {
    pub id: Option<i32>,
    pub name: String,
    pub seed: String,
    pub pubkey: String,
    pub passphrase: String,
    pub balance: Option<u64>,
}

impl Account {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Result<Self, DatabaseError> {
        let name = account_name_generator(&conn)?;
        let seed_phrase = secure_phrase_generator()?;
        let passphrase = secure_phrase_generator()?;
        let pubkey = pubkey_from_keypair_generator(&seed_phrase, &passphrase)?;
        let account = Account {
            id: None,
            name,
            seed: seed_phrase,
            pubkey,
            passphrase,
            balance: None,
        };
        insert_account(&conn, &account)?;
        Ok(account)
    }

    pub fn pubkey_display(&self) -> SharedString {
        let input_string = self.pubkey.clone();

        // Get the first 5 characters
        let first_part = &input_string[0..5];
        // Get the last 4 characters
        let last_part = &input_string[input_string.len() - 4..];

        // Combine with "..."
        let combined_string = format!("{}...{}", first_part, last_part);

        SharedString::from(combined_string)
    }

    pub fn pubkey(&self) -> Result<Pubkey, ParsePubkeyError> {
        let pubkey = Pubkey::from_str(&self.pubkey)?;
        Ok(pubkey)
    }

    pub fn balance_in_sol(&self) -> f64 {
        lamports_to_sol(self.balance.unwrap_or_else(|| 0u64))
    }

    pub fn account_keypair(&self) -> Result<Keypair, Box<dyn Error>> {
        let keypair =
            keypair::keypair_from_seed_phrase_and_passphrase(&*self.seed, &*self.passphrase)?;
        Ok(keypair)
    }
}

// Function to insert a new account into the accounts table
pub fn insert_account(
    conn: &Arc<Mutex<Connection>>,
    account: &Account,
) -> Result<usize, DatabaseError> {
    let conn_binding = conn.lock().unwrap();
    conn_binding
        .execute(
            "INSERT INTO accounts (name, seed, pubkey, passphrase) VALUES (?1, ?2, ?3, ?4)",
            params![
                &account.name,
                &account.seed,
                &account.pubkey,
                &account.passphrase,
            ],
        )
        .map_err(DatabaseError::from)
}

// Function to retrieve all accounts from the accounts table
pub fn get_accounts(conn: Arc<Mutex<Connection>>) -> Result<Vec<Account>, DatabaseError> {
    let conn_binding = conn.lock().unwrap();
    let query = "SELECT id, name, seed, pubkey, passphrase, balance FROM accounts";
    let mut stmt = conn_binding.prepare(query)?;
    let account_iter = stmt.query_map([], |row| {
        Ok(Account {
            id: row.get(0)?,
            name: row.get(1)?,
            seed: row.get(2)?,
            pubkey: row.get(3)?,
            passphrase: row.get(4)?,
            balance: row.get(5)?,
        })
    })?;

    let mut accounts = Vec::new();
    for account_result in account_iter {
        accounts.push(account_result?);
    }
    Ok(accounts)
}

fn account_name_generator(conn: Arc<Mutex<Connection>>) -> Result<String, DatabaseError> {
    let accounts_count = get_accounts(conn)?.len();
    let name = if accounts_count > 0 {
        format!("Account {}", accounts_count + 1)
    } else {
        "Main Account".to_string()
    };
    Ok(name)
}

fn secure_phrase_generator() -> Result<String, MnemonicError> {
    let mnemonic_phrase = Mnemonic::generate(12)?;
    let secure_phrase = mnemonic_phrase.words().collect::<Vec<&str>>().join(" ");
    Ok(secure_phrase)
}

fn pubkey_from_keypair_generator(
    seed_phrase: &String,
    passphrase: &String,
) -> Result<String, Box<dyn StdError>> {
    let keypair = keypair::keypair_from_seed_phrase_and_passphrase(seed_phrase, passphrase)?;
    let pubkey = keypair.pubkey().to_string();
    Ok(pubkey)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::database_connection;

    // Helper function to set up a temporary in-memory database
    fn setup_test_db() -> Arc<Mutex<Connection>> {
        let conn = Arc::new(Mutex::new(database_connection().unwrap()));
        let conn_binding = conn.clone();
        let conn_clone = conn_binding.lock().unwrap();
        conn_clone
            .execute(
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
        conn
    }

    #[test]
    fn test_account_new() {
        let conn = setup_test_db();
        let account = Account::new(conn).unwrap();

        // Validate that the account properties are correctly generated
        assert!(!account.id.is_some());
        assert!(account.name.starts_with("Main Account") || account.name.starts_with("Account"));
        assert!(!account.seed.is_empty());
        assert!(!account.pubkey.is_empty());
        assert!(!account.passphrase.is_empty());
    }

    #[test]
    fn test_pubkey_display() {
        let account = Account {
            id: None,
            name: "Test".to_string(),
            seed: "test_seed".to_string(),
            pubkey: "123456789abcdef".to_string(),
            passphrase: "test_passphrase".to_string(),
            balance: Some(1000),
        };
        let display = account.pubkey_display();
        assert_eq!(display.as_str(), "12345...cdef");
    }

    #[test]
    fn test_balance_in_sol() {
        let account = Account {
            id: None,
            name: "Test".to_string(),
            seed: "test_seed".to_string(),
            pubkey: "pubkey".to_string(),
            passphrase: "test_passphrase".to_string(),
            balance: Some(1_000_000_000), // 1 SOL in lamports
        };
        assert_eq!(account.balance_in_sol(), 1.0);
    }

    #[test]
    fn test_insert_account_and_get_accounts() {
        let conn = setup_test_db(); // Set up the in-memory database
        let conn_clone = conn.clone();

        let account = Account {
            id: None,
            name: "Test".to_string(),
            seed: "test_seed".to_string(),
            pubkey: "test_pubkey".to_string(),
            passphrase: "test_passphrase".to_string(),
            balance: None,
        };

        // Test inserting an account
        let result = insert_account(conn_clone, &account).unwrap();
        assert_eq!(result, 1); // One row should be inserted

        // Test retrieving accounts
        let accounts = get_accounts(conn).unwrap();
        assert_eq!(accounts.len(), 1);

        // Validate the retrieved account
        let retrieved_account = accounts.last().unwrap();
        assert_eq!(retrieved_account.name, account.name);
        assert_eq!(retrieved_account.seed, account.seed);
        assert_eq!(retrieved_account.pubkey, account.pubkey);
        assert_eq!(retrieved_account.passphrase, account.passphrase);
        assert_eq!(retrieved_account.balance, account.balance);
    }

    #[test]
    fn test_account_name_generator() {
        let conn = setup_test_db();
        let conn_clone = conn.clone();

        Account::new(conn_clone).unwrap();
        let name = account_name_generator(conn).unwrap();
        assert_eq!(name, "Account 2");
    }

    #[test]
    fn test_pubkey_from_keypair_generator() {
        let result =
            pubkey_from_keypair_generator(&"mock_seed".to_string(), &"mock_passphrase".to_string());
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }
}
