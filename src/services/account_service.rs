use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};

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
        insert_account(&self.conn, &account)?;
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
        get_accounts(&self.conn)
    }
}
