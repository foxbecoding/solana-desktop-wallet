use std::str::FromStr;
use bip39::{Mnemonic};
use rusqlite::{params, Connection};
use slint::SharedString;
use solana_sdk::native_token::lamports_to_sol;
use solana_sdk::signature::{keypair, Keypair};
use solana_sdk::signer::Signer;
use solana_sdk::pubkey::{ParsePubkeyError, Pubkey};
use crate::database::errors::DatabaseError;

#[derive(Debug, Clone)]
pub struct Account {
    pub id: Option<i32>,
    pub name: String,
    pub seed: String,
    pub pubkey: String,
    passphrase: String,
    pub balance: Option<u64>
}

impl Account {
    pub fn new(conn: &Connection, name: String) -> Result<Self, DatabaseError> {
        let mnemonic_for_seed = Mnemonic::generate(12)?;
        let mnemonic_for_passphrase = Mnemonic::generate(12)?;
        let seed_phrase = mnemonic_for_seed.words().collect::<Vec<&str>>().join(" ");
        let passphrase = mnemonic_for_passphrase.words().collect::<Vec<&str>>().join(" ");
        let keypair = keypair::keypair_from_seed_phrase_and_passphrase(&seed_phrase, &passphrase)?;
        let pubkey = keypair.pubkey().to_string();
        let account = Account {
            id: None,
            name,
            seed: seed_phrase,
            pubkey,
            passphrase,
            balance: None
        };
        insert_account(conn, &account)?;
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

    pub fn format_pubkey(&self) -> Result<Pubkey, ParsePubkeyError> {
        let pubkey = Pubkey::from_str(&self.pubkey)?;
        Ok(pubkey)
    }

    pub fn balance(&self) -> f32 {
        lamports_to_sol(self.balance.unwrap_or_else(|| 0u64)) as f32
    }

    pub fn account_keypair(&self) -> Result<Keypair, Box <dyn std::error::Error>> {
        let keypair = keypair::keypair_from_seed_phrase_and_passphrase(&*self.seed, &*self.passphrase)?;
        Ok(keypair)
    }
}

// Function to insert a new account into the accounts table
pub fn insert_account(conn: &Connection, account: &Account) -> Result<usize, DatabaseError> {
    conn.execute(
        "INSERT INTO accounts (name, seed, pubkey, passphrase) VALUES (?1, ?2, ?3, ?4)",
        params![
            &account.name,
            &account.seed,
            &account.pubkey,
            &account.passphrase,
        ],
    ).map_err(DatabaseError::from)
}

// Function to retrieve all accounts from the accounts table
pub fn get_accounts(conn: &Connection) -> Result<Vec<Account>, DatabaseError> {
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