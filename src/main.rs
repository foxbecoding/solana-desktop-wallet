// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod database;
mod connection;

use slint::{include_modules as include_slint_modules};
use solana_sdk::pubkey::{ParsePubkeyError, Pubkey};
use crate::database::{account::{Account as AccountModel, get_accounts}};
use crate::app::{app::App as MainApp, errors::AppError};

include_slint_modules!();
fn main() -> Result<(), AppError> {
    init()?;
    Ok(())
}

fn init() -> Result<(), AppError> {
    set_backend_renderer();
    let conn = database::database_connection()?;
    let mut accounts = get_accounts(&conn)?;
    let has_accounts = !accounts.is_empty();

    if !has_accounts {
        let new_account_name = "Main Account".to_string();
        AccountModel::new(&conn, new_account_name)?;
        accounts = get_accounts(&conn)?;
    }

    let app = MainApp { accounts };
    // start_app(app)?;
    Ok(())
}

fn set_backend_renderer() {
    std::env::set_var("SLINT_BACKEND", "winit");
    std::env::set_var("SLINT_RENDERER", "skia");
}

fn set_accounts_balances(accounts: &Vec<AccountModel>) -> Result<Vec<AccountModel>, ParsePubkeyError> {
    let new_connection = connection::Connection::new();
    let connection = new_connection.connection();
    let mut accounts_pubkeys: Vec<Pubkey> = vec![];
    for account in accounts {
        let pubkey = account.format_pubkey()?;
        accounts_pubkeys.push(pubkey)
    }
    let sol_accounts = connection.get_multiple_accounts(&accounts_pubkeys)?;
    println!("{:#?}", sol_accounts);
    Ok(vec![])
}

fn start_app(app: MainApp) -> Result<(), AppError> {
    app.start()?;
    Ok(())
}