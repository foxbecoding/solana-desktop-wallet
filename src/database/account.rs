use bip39::{Mnemonic};
use rusqlite::{params, Connection};
use slint::SharedString;
use solana_sdk::keccak;
use solana_sdk::signature::keypair;
use solana_sdk::signer::Signer;
use crate::database::errors::DatabaseError;

#[derive(Debug, Clone)]
pub struct Account {
    pub id: Option<i32>,
    pub name: String,
    pub seed: String,
    pub pubkey: String,
}

impl Account {
    pub fn new(conn: &Connection, name: String) -> Result<Self, DatabaseError> {
        let mnemonic = Mnemonic::generate(12)?;
        let seed_phrase = mnemonic.words().collect::<Vec<&str>>().join(" ");
        let hashed_seed = seed_phrase_hasher(&seed_phrase);
        let keypair = keypair::keypair_from_seed(hashed_seed.as_bytes())?;
        let pubkey = keypair.pubkey().to_string();
        let account = Account {
            id: None,
            name,
            seed: seed_phrase,
            pubkey,
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

    pub fn seed_to_vec(&self) -> Vec<SharedString> {
        self.seed.split_whitespace().map(|word| SharedString::from(word)).collect()
    }
}


// Function to insert a new account into the accounts table
pub fn insert_account(conn: &Connection, account: &Account) -> Result<usize, DatabaseError> {
    conn.execute(
        "INSERT INTO accounts (name, seed, pubkey) VALUES (?1, ?2, ?3, ?4)",
        params![
            &account.name,
            &account.seed,
            &account.pubkey
        ],
    ).map_err(DatabaseError::from)
}

// Function to retrieve all accounts from the accounts table
pub fn get_accounts(conn: &Connection) -> Result<Vec<Account>, DatabaseError> {
    let query = "SELECT id, name, seed, pubkey FROM accounts";
    let mut stmt = conn.prepare(query)?;
    let account_iter = stmt.query_map([], |row| {
        Ok(Account {
            id: row.get(0)?,
            name: row.get(1)?,
            seed: row.get(2)?,
            pubkey: row.get(3)?
        })
    })?;

    let mut accounts = Vec::new();
    for account_result in account_iter {
        accounts.push(account_result?);
    }
    Ok(accounts)
}

fn seed_phrase_hasher(seed_phrase: &String) -> String {

    // Hash the seed input using Keccak
    let mut hasher = keccak::Hasher::default();
    hasher.hash(seed_phrase.as_bytes());
    let keccak_hash = hasher.result();

    // Convert the hash to an array and take the first 32 bytes
    let keccak_bytes: [u8; 32] = keccak_hash.to_bytes();

    // Encode the result as hex
    let seed = &hex::encode(&keccak_bytes)[0..32];
    seed.to_string()
}