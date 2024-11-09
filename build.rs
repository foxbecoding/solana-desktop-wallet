use std::env;
use std::env::VarError;
use slint_build::CompileError;
use rusqlite::{Connection, Result, Error as RusqliteError};
use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum BuildError {
    #[error("Environment variable error: {0}")]
    VarError(#[from] VarError),

    #[error("Compile error: {0}")]
    CompileError(#[from] CompileError),

    #[error("Database error: {0}")]
    RusqliteError(#[from] RusqliteError),
}

fn main() -> Result<(), BuildError> {
    // Load environment variables from a .env file
    dotenv::dotenv().ok();
    build_app_ui()?;
    let conn = database_connection()?;
    create_wallets_table(&conn)?;
    Ok(())
}

fn build_app_ui() -> Result<(), BuildError> {
    let app_entry = env::var("APP_ENTRY")?;
    let app_style = env::var("APP_STYLE")?;
    let config = slint_build::CompilerConfiguration::new().with_style(app_style.into());
    slint_build::compile_with_config(app_entry, config)?;
    Ok(())
}

pub fn database_connection() -> Result<Connection, BuildError> {
    let database_path = env::var("DATABASE_PATH")?;
    let conn = Connection::open(database_path)?;
    Ok(conn)
}

// Function to create the wallets table if it doesn't exist
pub fn create_wallets_table(conn: &Connection) -> Result<(), BuildError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS wallets (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            seed TEXT NOT NULL,
            public_key TEXT NOT NULL,
            is_passphrase_protected BOOLEAN NOT NULL
        )",
        [],
    )?;
    Ok(())
}