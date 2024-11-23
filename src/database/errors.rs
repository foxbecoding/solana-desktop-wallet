use thiserror::Error;
use rusqlite::{Error as RusqliteError};
use bip39::{Error as MnemonicError};
use serde::de::StdError;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database error: {0}")]
    RusqliteError(#[from] RusqliteError),

    #[error("Mnemonic error: {0}")]
    MnemonicError(#[from] MnemonicError),

    #[error("Other error: {0}")]
    Other(#[from] Box<dyn StdError>),
}