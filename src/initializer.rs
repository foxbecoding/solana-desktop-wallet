use crate::app::errors::AppError;
use crate::database::account::{get_accounts, Account as AccountModel};

pub fn init() -> Result<(), AppError> {
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
