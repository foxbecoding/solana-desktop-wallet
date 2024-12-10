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

    fn setup_db_connection() -> Arc<Mutex<Connection>> {
        let conn = database_connection().unwrap();
        Arc::new(Mutex::new(conn))
    }
}
