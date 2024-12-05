use crate::app::{errors::AppError, App};
use crate::connection::Connection;
use crate::database::account::{get_accounts, Account as AccountModel};
use crate::database::database_connection;
use rusqlite::Connection as SqliteConnection;
use solana_sdk::pubkey::Pubkey;
use std::{env, error::Error};

pub fn run(conn: &SqliteConnection) -> Result<(), AppError> {
    //let conn = database_connection()?;
    set_backend_renderer();
    let mut accounts = get_accounts(conn)?;
    accounts = set_accounts_balances(accounts.clone())?;
    let has_accounts = !accounts.is_empty();

    if !has_accounts {
        AccountModel::new(conn)?;
        accounts = get_accounts(conn)?;
    }

    let app = App { accounts };
    start_app(app)?;
    Ok(())
}

fn set_backend_renderer() {
    env::set_var("SLINT_BACKEND", "winit");
    env::set_var("SLINT_RENDERER", "skia");
}

fn set_accounts_balances(accounts: Vec<AccountModel>) -> Result<Vec<AccountModel>, Box<dyn Error>> {
    let new_connection = Connection::new();
    let connection = new_connection.connection();

    let accounts_pubkeys: Vec<Pubkey> = accounts
        .iter()
        .map(|account| account.pubkey())
        .collect::<Result<Vec<_>, _>>()?;

    let sol_accounts = connection.get_multiple_accounts(&accounts_pubkeys)?;

    let mut updated_accounts = accounts;

    for (account, sol_account) in updated_accounts.iter_mut().zip(sol_accounts.iter()) {
        if let Some(sol_account) = sol_account {
            account.balance = Some(sol_account.lamports);
        } else {
            account.balance = None;
        }
    }

    Ok(updated_accounts)
}

fn start_app(app: App) -> Result<(), AppError> {
    app.start()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to set up a temporary in-memory database
    fn setup_test_db() -> SqliteConnection {
        let conn = database_connection().unwrap();
        conn.execute(
            "CREATE TABLE accounts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                seed TEXT NOT NULL,
                pubkey TEXT NOT NULL,
                passphrase TEXT NOT NULL,
                balance INTEGER
            )",
            [],
        )
        .unwrap();
        conn
    }

    #[test]
    fn test_set_backend_renderer() {
        // Call the function
        set_backend_renderer();

        // Check that the environment variables are set correctly
        assert_eq!(env::var("SLINT_BACKEND").unwrap(), "winit");
        assert_eq!(env::var("SLINT_RENDERER").unwrap(), "skia");
    }

    #[test]
    fn test_set_accounts_balances() {
        // Mock accounts
        let mock_accounts = vec![
            AccountModel {
                id: None,
                name: "Test".to_string(),
                seed: "test_seed".to_string(),
                pubkey: Pubkey::new_unique().to_string(),
                passphrase: "test_passphrase".to_string(),
                balance: None,
            },
            AccountModel {
                id: None,
                name: "Test".to_string(),
                seed: "test_seed".to_string(),
                pubkey: Pubkey::new_unique().to_string(),
                passphrase: "test_passphrase".to_string(),
                balance: None,
            },
        ];

        // Mock connection (requires setting up a mock `Connection` with a library like `mockall`).
        // Assuming we use a mock client to simulate `get_multiple_accounts`.
        let updated_accounts = set_accounts_balances(mock_accounts.clone());

        // Check results (adjust assertions based on actual mock behavior).
        assert!(updated_accounts.is_ok());
        let updated_accounts = updated_accounts.unwrap();
        assert!(updated_accounts.iter().all(|a| a.balance.is_some()));
    }

    #[test]
    fn test_start_app() {
        // Mock an app instance
        let app = App { accounts: vec![] };

        // Check if starting the app works without issues
        let result = start_app(app);
        assert!(result.is_ok(), "start_app failed");
    }

    #[test]
    fn test_run_successful() {
        // Set up a mock database
        let conn = setup_test_db();

        // Create a mock `get_accounts` implementation
        AccountModel::new(&conn).unwrap();

        // Call the `run` function
        let result = run(&conn);

        // Assert the function completes successfully
        assert!(result.is_ok(), "run failed with error: {:?}", result.err());
    }
}
