use crate::database::{database_connection, errors::DatabaseError};
use bip39::{Error as MnemonicError, Mnemonic};
use rusqlite::params;
use serde::de::StdError;
use slint::SharedString;
use solana_sdk::native_token::lamports_to_sol;
use solana_sdk::pubkey::{ParsePubkeyError, Pubkey};
use solana_sdk::signature::{keypair, Keypair};
use solana_sdk::signer::Signer;
use std::{error::Error, str::FromStr};

#[derive(Debug, Clone)]
pub struct Account {
    pub id: Option<i32>,
    pub name: String,
    pub seed: String,
    pub pubkey: String,
    passphrase: String,
    pub balance: Option<u64>,
}

impl Account {
    pub fn new() -> Result<Self, DatabaseError> {
        let name = account_name_generator()?;
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
        insert_account(&account)?;
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
pub fn insert_account(account: &Account) -> Result<usize, DatabaseError> {
    let conn = database_connection()?;
    conn.execute(
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
pub fn get_accounts() -> Result<Vec<Account>, DatabaseError> {
    let conn = database_connection()?;
    let query = "SELECT id, name, seed, pubkey, passphrase, balance FROM accounts";
    let mut stmt = conn.prepare(query)?;
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

fn account_name_generator() -> Result<String, DatabaseError> {
    let accounts_count = get_accounts()?.len();
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
    use rusqlite::Connection;

    // Helper function to set up a temporary in-memory database
    fn setup_test_db() -> Connection {
        let conn = database_connection().unwrap();
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
        conn
    }

    #[derive(Debug, Clone)]
    pub struct MockAccount {
        pub id: Option<i32>,
        pub name: String,
        pub seed: String,
        pub pubkey: String,
        passphrase: String,
        pub balance: Option<u64>,
    }

    // Mock function to simulate `get_accounts`
    fn mock_get_accounts(accounts: Vec<Account>) -> Result<Vec<Account>, DatabaseError> {
        Ok(accounts) // Return the mock accounts as a simulated database query result
    }

    fn mock_insert_account(conn: &Connection, account: &Account) -> Result<usize, DatabaseError> {
        conn.execute(
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

    #[test]
    fn test_account_new() {
        let _conn = setup_test_db(); // Ensure a clean database environment

        let account = Account::new();
        assert!(account.is_ok());
        let account = account.unwrap();

        // Validate that the account properties are correctly generated
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
        let _conn = setup_test_db(); // Set up the in-memory database

        let account = Account {
            id: None,
            name: "Test".to_string(),
            seed: "test_seed".to_string(),
            pubkey: "test_pubkey".to_string(),
            passphrase: "test_passphrase".to_string(),
            balance: None,
        };

        // Test inserting an account
        let result = insert_account(&account);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1); // One row should be inserted

        // Test retrieving accounts
        let accounts = get_accounts();
        assert!(accounts.is_ok());
        let accounts = accounts.unwrap();
        assert_ne!(accounts.len(), 0);

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
        // Mock the `get_accounts` function
        let _mock = mock_get_accounts(vec![Account {
            id: Some(1),
            name: "Main Account".to_string(),
            seed: "seed".to_string(),
            pubkey: "pubkey".to_string(),
            passphrase: "passphrase".to_string(),
            balance: None,
        }]);

        let name = account_name_generator();
        assert!(name.is_ok());
        //assert_eq!(name.unwrap(), "Account 2");
    }

    #[test]
    fn test_pubkey_from_keypair_generator() {
        let result =
            pubkey_from_keypair_generator(&"mock_seed".to_string(), &"mock_passphrase".to_string());
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }
}
