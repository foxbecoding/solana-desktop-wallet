use rusqlite::{params, Connection};
use slint::SharedString;
use std::sync::{Arc, Mutex};

use crate::database::{
    account::{get_accounts, Account},
    errors::DatabaseError,
};

pub struct AccountService {
    conn: Arc<Mutex<Connection>>,
}

impl AccountService {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    pub fn create_account(&self) -> Result<Account, DatabaseError> {
        let account = Account::new(self.conn.clone())?;
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

    pub fn get_account_balance(&self, account: &Account) -> f64 {
        account.balance_in_sol()
    }

    pub fn generate_pubkey_display(&self, account: &Account) -> SharedString {
        account.pubkey_display()
    }
}
