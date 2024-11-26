// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;
use slint::{include_modules as include_slint_modules};
use solana_sdk::pubkey::{Pubkey};

mod app;
mod connection;
mod database;

use crate::database::{account::{Account as AccountModel, get_accounts}};
use crate::app::errors::AppError;

include_slint_modules!();
fn main() -> Result<(), AppError> {
    init()?;
    Ok(())
}

fn init() -> Result<(), AppError> {
    set_backend_renderer();
    let mut accounts = get_accounts()?;
    accounts = set_accounts_balances(accounts.clone())?;
    let has_accounts = !accounts.is_empty();

    if !has_accounts {
        AccountModel::new()?;
        accounts = get_accounts()?;
    }

    let app = app::App { accounts };
    start_app(app)?;
    Ok(())
}

fn set_backend_renderer() {
    std::env::set_var("SLINT_BACKEND", "winit");
    std::env::set_var("SLINT_RENDERER", "skia");
}

fn set_accounts_balances(accounts: Vec<AccountModel>) -> Result<Vec<AccountModel>, Box<dyn Error>> {
    let new_connection = connection::Connection::new();
    let connection = new_connection.connection();

    let accounts_pubkeys: Vec<Pubkey> = accounts.iter()
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

fn start_app(app: app::App) -> Result<(), AppError> {
    app.start()?;
    Ok(())
}