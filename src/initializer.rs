use crate::app::{errors::AppError, App};
use crate::connection::Connection;
use crate::database::account::{get_accounts, Account as AccountModel};
use crate::database::database_connection;
use solana_sdk::pubkey::Pubkey;
use std::error::Error;

pub fn run() -> Result<(), AppError> {
    let conn = database_connection()?;
    set_backend_renderer();
    let mut accounts = get_accounts(&conn)?;
    accounts = set_accounts_balances(accounts.clone())?;
    let has_accounts = !accounts.is_empty();

    if !has_accounts {
        AccountModel::new(&conn)?;
        accounts = get_accounts(&conn)?;
    }

    let app = App { accounts };
    start_app(app)?;
    Ok(())
}

fn set_backend_renderer() {
    std::env::set_var("SLINT_BACKEND", "winit");
    std::env::set_var("SLINT_RENDERER", "skia");
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
