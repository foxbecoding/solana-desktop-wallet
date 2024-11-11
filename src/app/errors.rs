use slint::PlatformError;
use thiserror::Error;
use crate::database::errors::DatabaseError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Platform error: {0}")]
    PlatformError(#[from] PlatformError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),
}