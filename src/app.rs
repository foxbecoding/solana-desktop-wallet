pub mod callback_manager;
pub mod errors;
pub mod global_manager;
use crate::app::{
    callback_manager::CallbackManager, errors::AppError, global_manager::GlobalManager,
};
use crate::database::account::Account;
use crate::slint_generatedApp::{App as SlintApp, View as SlintViewEnum};
use rusqlite::Connection;
use slint::ComponentHandle;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct App {
    pub accounts: Vec<Account>,
    pub conn: Arc<Mutex<Connection>>,
}

impl App {
    pub fn start(&self) -> Result<(), AppError> {
        self.run_app()?;
        Ok(())
    }

    fn run_app(&self) -> Result<(), AppError> {
        let app = SlintApp::new()?;
        let weak_app = app.as_weak().unwrap();
        self.run_managers(weak_app)?;

        if !cfg!(test) {
            app.run()?;
        }

        Ok(())
    }

    fn run_managers(&self, app_instance: SlintApp) -> Result<(), AppError> {
        let conn = self.conn.clone();
        GlobalManager::new(
            conn.clone(),
            app_instance.clone_strong(),
            self.accounts.clone(),
        )
        .run()?;
        CallbackManager::new(conn, app_instance).run()?;
        Ok(())
    }
}

pub fn app_view_selector(view: String) -> SlintViewEnum {
    match view.as_str() {
        "Wallet" => SlintViewEnum::Wallet,
        "Collections" => SlintViewEnum::Collections,
        "Swap" => SlintViewEnum::Swap,
        "Explore" => SlintViewEnum::Explore,
        "Settings" => SlintViewEnum::Settings,
        "Accounts" => SlintViewEnum::Accounts,
        _ => SlintViewEnum::Wallet,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::database_connection;

    fn setup_test_db() -> Arc<Mutex<Connection>> {
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

    fn mock_run_app() -> Result<(), AppError> {
        let app = SlintApp::new().unwrap();
        let weak_app = app.as_weak().unwrap();
        mock_run_managers(weak_app)
    }

    fn mock_run_managers(app_instance: SlintApp) -> Result<(), AppError> {
        let conn = setup_test_db();
        let accounts = vec![Account {
            id: Some(1),
            name: "Main Account".to_string(),
            seed: "dummy_seed".to_string(),
            pubkey: "dummy_pubkey".to_string(),
            passphrase: "dummy_passphrase".to_string(),
            balance: Some(100),
        }];

        GlobalManager::new(conn.clone(), app_instance.clone_strong(), accounts)
            .run()
            .unwrap();
        CallbackManager::new(conn, app_instance).run().unwrap();
        Ok(())
    }

    fn test_app_start() {
        let conn = setup_test_db();
        let accounts = vec![Account {
            id: Some(1),
            name: "Main Account".to_string(),
            seed: "dummy_seed".to_string(),
            pubkey: "dummy_pubkey".to_string(),
            passphrase: "dummy_passphrase".to_string(),
            balance: Some(100),
        }];

        let app = App { conn, accounts };

        // Test that `start` runs without errors.
        let result = app.start();
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_app() {
        let _conn = setup_test_db();
        let result = mock_run_app();
        assert!(result.is_ok());
    }

    #[test]
    fn test_run_managers() {
        let _conn = setup_test_db();
        // Create a mock instance of SlintApp
        let slint_app = SlintApp::new().unwrap();

        let result = mock_run_managers(slint_app);
        assert!(result.is_ok());
    }

    #[test]
    fn test_app_view_selector() {
        assert_eq!(
            app_view_selector("Wallet".to_string()),
            SlintViewEnum::Wallet
        );
        assert_eq!(
            app_view_selector("Collections".to_string()),
            SlintViewEnum::Collections
        );
        assert_eq!(app_view_selector("Swap".to_string()), SlintViewEnum::Swap);
        assert_eq!(
            app_view_selector("Explore".to_string()),
            SlintViewEnum::Explore
        );
        assert_eq!(
            app_view_selector("Settings".to_string()),
            SlintViewEnum::Settings
        );
        assert_eq!(
            app_view_selector("Accounts".to_string()),
            SlintViewEnum::Accounts
        );
        assert_eq!(
            app_view_selector("Unknown".to_string()),
            SlintViewEnum::Wallet
        );
    }
}
