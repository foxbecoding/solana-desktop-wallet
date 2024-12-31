use crate::database::errors::DatabaseError;
use anyhow::Error as AnyhowError;
use serde::de::StdError;
use slint::PlatformError;
use solana_sdk::pubkey::ParsePubkeyError;
use thiserror::Error;
use webbrowser::ParseBrowserError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Platform error: {0}")]
    PlatformError(#[from] PlatformError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),

    #[error("Parse browser error: {0}")]
    ParseBrowserError(#[from] ParseBrowserError),

    #[error("Parse pubkey error: {0}")]
    ParsePubkeyError(#[from] ParsePubkeyError),

    #[error("Anyhow error: {0}")]
    AnyhowError(#[from] AnyhowError),

    #[error("Other error: {0}")]
    Other(#[from] Box<dyn StdError>),

    #[error("No account selected")]
    NoAccountSelected,
}
