extern crate thiserror;

use thiserror::Error;
use rusqlite::{Error as RusqliteError};

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database error: {0}")]
    RusqliteError(#[from] RusqliteError),
}