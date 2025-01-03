use std::env;
use std::env::VarError;
use slint_build::CompileError;
use rusqlite::{Connection, Result, Error as RusqliteError};
use thiserror::Error as ThisError;

const FORCE_REBUILD: bool = false;

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
    force_rebuild();
    build_app_ui()?;
    create_db_tables()?;
    Ok(())
}

fn force_rebuild() {
    let rebuild = FORCE_REBUILD;
    if rebuild {
        println!("cargo:rerun-if-changed=force_rebuild");
    }
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

// Function to create the accounts table if it doesn't exist
pub fn create_accounts_table(conn: &Connection) -> Result<(), BuildError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS accounts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            seed TEXT NOT NULL,
            pubkey TEXT NOT NULL,
            passphrase TEXT NOT NULL,
            balance INTEGER NULL
        )",
        [],
    )?;
    Ok(())
}

pub fn create_cache_table(conn: &Connection) -> Result<(), BuildError> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cache (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
        [],
    )?;
    Ok(())
}

pub fn create_db_tables() -> Result<(), BuildError> {
    let conn = database_connection()?;
    create_accounts_table(&conn)?;
    create_cache_table(&conn)?;
    Ok(())
}