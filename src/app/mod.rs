use slint::{ComponentHandle, PlatformError};
use thiserror::Error;
use crate::database::{
    database_connection,
    errors::DatabaseError,
    wallet::{Wallet, create_wallets_table, insert_wallet},
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
    pub fn start() -> Result<(), AppError> {
        let conn = database_connection()?;
        create_wallets_table(&conn)?;

        // Insert a new wallet
        let wallet = Wallet {
            id: None,
            name: String::from("Main Wallet"),
            seed: String::from("random seed phrase"),
        };
        insert_wallet(&conn, &wallet)?;

        let app = crate::App::new()?;
        app.run()?;
        Ok(())
    }
}