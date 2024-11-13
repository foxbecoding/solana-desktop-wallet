use serde::de::StdError;
use slint::PlatformError;
use thiserror::Error;
use webbrowser::ParseBrowserError;
use crate::database::errors::DatabaseError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Platform error: {0}")]
    PlatformError(#[from] PlatformError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),

    #[error("Parse browser error: {0}")]
    ParseBrowserError(#[from] ParseBrowserError),

    #[error("Other error: {0}")]
    Other(#[from] Box<dyn StdError>),

    #[error("No account selected")]
    NoAccountSelected,
}