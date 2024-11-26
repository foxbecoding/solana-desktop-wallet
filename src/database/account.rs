use std::{error::Error, str::FromStr};
use bip39::{Mnemonic, Error as MnemonicError};
use rusqlite::{params};
use slint::SharedString;
use solana_sdk::native_token::lamports_to_sol;
use solana_sdk::signature::{keypair, Keypair};
use solana_sdk::signer::Signer;
use solana_sdk::pubkey::{ParsePubkeyError, Pubkey};
use crate::database::{database_connection, errors::DatabaseError};

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
    pub fn new() -> Result<Self, DatabaseError> {
        let name = account_name_generator()?;
        let seed_phrase = secure_phrase_generator()?;
        let passphrase = secure_phrase_generator()?;
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

    pub fn account_keypair(&self) -> Result<Keypair, Box <dyn Error>> {
        let keypair = keypair::keypair_from_seed_phrase_and_passphrase(&*self.seed, &*self.passphrase)?;
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
    ).map_err(DatabaseError::from)
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

fn secure_phrase_generator() -> Result<String, MnemonicError>{
    let mnemonic_phrase = Mnemonic::generate(12)?;
    let secure_phrase = mnemonic_phrase.words().collect::<Vec<&str>>().join(" ");
    Ok(secure_phrase)
}

