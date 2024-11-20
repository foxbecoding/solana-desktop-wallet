use slint::ComponentHandle;
use solana_sdk::msg;
use crate::get_accounts;
use crate::database::{database_connection, errors::DatabaseError};

pub struct CallbackManager<'a>  {
    app_instance: &'a crate::App
}

impl<'a> CallbackManager<'a> {
    pub fn new(app_instance: &'a crate::App) -> Self {
        CallbackManager { app_instance }
    }

    pub fn run(&self) -> Result<(), DatabaseError> {
        self.init_handlers()?;
        Ok(())
    }

    fn init_handlers(&self) -> Result<(), DatabaseError> {
        self.view_account_handler();
        self.add_account_handler()?;
        Ok(())
    }

    fn view_account_handler(&self) {
        self.app_instance.global::<crate::AccountManager>().on_view_account(|pubkey| {
            let url = format!("https://solscan.io/account/{}", pubkey);

            if webbrowser::open(url.as_str()).is_ok() {
                msg!("Opened '{}' in your default web browser.", pubkey);
            } else {
                msg!("Failed to open '{}'.", pubkey);
            }
        });
    }

    fn add_account_handler(&self) -> Result<(), DatabaseError> {
        self.app_instance.global::<crate::AccountManager>().on_add_account(|| {
            let db_conn = database_connection()?;
            let accounts_count = get_accounts(&db_conn)?.len();
            println!("{accounts_count}");
        });
        Ok(())
    }
}