use bip39::{Error as MnemonicError, Mnemonic};
use rusqlite::{params, Connection};
use solana_sdk::signature::keypair;
use solana_sdk::signer::Signer;
use std::{
    error::Error as StdError,
    sync::{Arc, Mutex},
};

use crate::database::{account::Account, errors::DatabaseError};

pub struct AccountService {
    conn: Arc<Mutex<Connection>>,
}

impl AccountService {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    pub fn create_account(&self) -> Result<Account, DatabaseError> {
        let name = self.account_name_generator()?;
        let seed_phrase = self.secure_phrase_generator()?;
        let passphrase = self.secure_phrase_generator()?;
        let pubkey = self.pubkey_from_keypair_generator(&seed_phrase, &passphrase)?;
        let account = Account {
            id: None,
            name,
            seed: seed_phrase,
            pubkey,
            passphrase,
            balance: None,
        };
        self.insert_account(&account)?;
        Ok(account)
    }

    pub fn insert_account(&self, account: &Account) -> Result<usize, DatabaseError> {
        let conn_binding = self.conn.lock().unwrap();
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

    pub fn get_all_accounts(&self) -> Result<Vec<Account>, DatabaseError> {
        let conn_binding = self.conn.lock().unwrap();
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

    fn account_name_generator(&self) -> Result<String, DatabaseError> {
        let accounts_count = self.get_all_accounts()?.len();
        Ok(if accounts_count > 0 {
            format!("Account {}", accounts_count + 1)
        } else {
            "Main Account".to_string()
        })
    }

    fn secure_phrase_generator(&self) -> Result<String, MnemonicError> {
        let mnemonic_phrase = Mnemonic::generate(12)?;
        Ok(mnemonic_phrase.words().collect::<Vec<&str>>().join(" "))
    }

    fn pubkey_from_keypair_generator(
        &self,
        seed_phrase: &String,
        passphrase: &String,
    ) -> Result<String, Box<dyn StdError>> {
        let keypair = keypair::keypair_from_seed_phrase_and_passphrase(seed_phrase, passphrase)?;
        Ok(keypair.pubkey().to_string())
    }
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
    fn test_insert_account_and_get_accounts() {
        let conn = setup_test_db(); // Set up the in-memory database

        let account = Account {
            id: None,
            name: "Test".to_string(),
            seed: "test_seed".to_string(),
            pubkey: "test_pubkey".to_string(),
            passphrase: "test_passphrase".to_string(),
            balance: None,
        };

        // Test inserting an account
        let result = insert_account(&conn, &account).unwrap();
        assert_eq!(result, 1); // One row should be inserted

        // Test retrieving accounts
        let accounts = get_accounts(&conn).unwrap();
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
        Account::new(conn.clone()).unwrap();
        let name = account_name_generator(&conn).unwrap();
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
