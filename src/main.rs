// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod database;

use slint::{include_modules as include_slint_modules};
use crate::database::{account::{Account as AccountModel, get_accounts}};
use crate::app::{app::App as MainApp, errors::AppError};

include_slint_modules!();
fn main() -> Result<(), AppError> {
    init()?;
    Ok(())
}

fn init() -> Result<(), AppError> {
    set_backend_renederer();
    let conn = database::database_connection()?;
    let mut accounts = get_accounts(&conn)?;
    let has_accounts = !accounts.is_empty();

    if !has_accounts {
        let new_account_name = "Main Account".to_string();
        AccountModel::new(&conn, new_account_name)?;
        accounts = get_accounts(&conn)?;
    }

    let app = MainApp { accounts };
    start_app(app)?;
    Ok(())
}

fn set_backend_renederer() {
    std::env::set_var("SLINT_BACKEND", "winit");
    std::env::set_var("SLINT_RENDERER", "skia");
}

fn start_app(app: MainApp) -> Result<(), AppError> {
    app.start()?;
    Ok(())
}