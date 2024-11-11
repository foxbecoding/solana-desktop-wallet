// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod database;

use std::error::Error;
use slint::{include_modules as include_slint_modules};
use crate::database::{account::{Account as AccountModel}, errors::DatabaseError};
use crate::app::{app::App as MainApp, errors::AppError};

include_slint_modules!();
fn main() -> Result<(), AppError> {
    init()?;
    Ok(())
}

fn init() -> Result<(), AppError> {
    let accounts = init_accounts()?;
    let has_accounts = !accounts.is_empty();
    let app = MainApp { accounts, show_app_content: has_accounts };
    init_app(app)?;
    Ok(())
}

fn init_accounts() -> Result<Vec<AccountModel>, DatabaseError> {
    let conn = database::database_connection()?;
    let accounts = database::account::get_accounts(&conn)?;
    Ok(accounts)
}

fn init_app(app: MainApp) -> Result<(), AppError> {
    app.start()?;
    Ok(())
}