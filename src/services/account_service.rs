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
        get_accounts(&self.conn)
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
