use crate::app::{errors::AppError, App};
use crate::connection::Connection;
use crate::database::account::Account;
use crate::services::account_service::AccountService;
use rusqlite::Connection as SqliteConnection;
use solana_sdk::pubkey::Pubkey;
use std::{
    env,
    error::Error,
    sync::{Arc, Mutex},
};

pub fn run(conn: Arc<Mutex<SqliteConnection>>) -> Result<(), AppError> {
    set_backend_renderer();
    let account_service = AccountService::new(conn);
    let mut accounts = account_service.get_all_accounts()?;
    accounts = set_accounts_balances(accounts.clone())?;
    let has_accounts = !accounts.is_empty();

    if !has_accounts {
        account_service.create_account()?;
        accounts = account_service.get_all_accounts()?;
    }

    let app = App { accounts, conn };
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
    use crate::database::database_connection;

    // Helper function to set up a temporary in-memory database
    fn setup_test_db() -> Arc<Mutex<SqliteConnection>> {
        let conn = Arc::new(Mutex::new(database_connection().unwrap()));
        let conn_clone_binding = conn.clone();
        let conn_clone = conn_clone_binding.lock().unwrap();

        conn_clone
            .execute(
                "CREATE TABLE cache (key TEXT PRIMARY KEY, value TEXT NOT NULL)",
                [],
            )
            .unwrap();

        conn_clone
            .execute(
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
        let conn = setup_test_db();
        AccountModel::new(conn.clone()).unwrap();
        let accounts = get_accounts(&conn).unwrap();

        // Mock connection (requires setting up a mock `Connection` with a library like `mockall`).
        // Assuming we use a mock client to simulate `get_multiple_accounts`.
        let updated_accounts = set_accounts_balances(accounts.clone());

        // Check results (adjust assertions based on actual mock behavior).
        assert!(updated_accounts.is_ok());
        let updated_accounts = updated_accounts.unwrap();

        assert!(updated_accounts.iter().all(|a| a.balance.is_none()));
    }

    //#[test]
    //fn test_start_app() {
    //    let conn = setup_test_db();
    //
    //    AccountModel::new(conn.clone()).unwrap();
    //    let accounts = get_accounts(&conn).unwrap();
    //
    //    // Mock an app instance
    //    let app = App { accounts, conn };
    //
    //    // Check if starting the app works without issues
    //    let result = start_app(app);
    //    assert!(result.is_ok(), "start_app failed");
    //}

    #[test]
    fn test_run_successful() {
        // Set up a mock database
        let conn = setup_test_db();

        // Create a mock `get_accounts` implementation
        AccountModel::new(conn.clone()).unwrap();

        // Call the `run` function
        let result = run(conn);

        // Assert the function completes successfully
        assert!(result.is_ok(), "run failed with error: {:?}", result.err());
    }
}
