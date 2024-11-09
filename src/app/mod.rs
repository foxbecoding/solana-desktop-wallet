use slint::{ComponentHandle, PlatformError};
use thiserror::Error;
use crate::database::{
    database_connection,
    errors::DatabaseError,
    wallet::{Wallet, insert_wallet},
};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Platform error: {0}")]
    PlatformError(#[from] PlatformError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),
}

pub struct App {}

impl App {
    pub fn start(&self) -> Result<(), AppError> {
        self.run_app()?;
        Ok(())
    }

    fn run_app(&self) -> Result<(), AppError> {
        let app = crate::App::new()?;
        app.run()?;
        Ok(())
    }
}