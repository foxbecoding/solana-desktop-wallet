use slint::SharedString;
use solana_sdk::native_token::lamports_to_sol;
use solana_sdk::pubkey::{ParsePubkeyError, Pubkey};
use solana_sdk::signature::{keypair, Keypair};
use std::{error::Error, str::FromStr};

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
    pub fn pubkey_display(&self) -> SharedString {
        let input_string = self.pubkey.clone();
        let first_part = &input_string[0..5];
        let last_part = &input_string[input_string.len() - 4..];
        SharedString::from(format!("{}...{}", first_part, last_part))
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
