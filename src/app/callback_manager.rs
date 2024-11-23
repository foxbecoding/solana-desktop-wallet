use slint::{ComponentHandle};
use solana_sdk::msg;
use crate::database::{database_connection, errors::DatabaseError, account::{Account, get_accounts}};
use crate::app::global_manager::GlobalManager;
use crate::slint_generatedApp::{App as SlintApp, AccountManager};

pub struct CallbackManager {
    app_instance: SlintApp,
}

impl CallbackManager {
    pub fn new(app_instance: SlintApp) -> Self {
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
        self.app_instance.global::<crate::AccountManager>().on_view_account(move |pubkey| {
            let url = format!("https://solscan.io/account/{}", pubkey);

            if webbrowser::open(url.as_str()).is_ok() {
                msg!("Opened '{}' in your default web browser.", pubkey);
            } else {
                msg!("Failed to open '{}'.", pubkey);
            }
        });
    }

    fn add_account_handler(&self) -> Result<(), DatabaseError> {
        let app = self.app_instance.clone_strong();
        let weak_app = app.as_weak().unwrap();
        app.global::<crate::AccountManager>().on_add_account(move || {
            let result = (|| -> Result<(), DatabaseError> { // Establish db connection
                let db_conn = database_connection()?;

                // get accounts count
                let accounts_count = get_accounts(&db_conn)?.len();

                // set new account name
                let new_account_name = format!("Account {}", accounts_count + 1);

                // insert into DB
                Account::new(&db_conn, new_account_name)?;

                // set accounts in app With Global Manager
                let accounts = get_accounts(&db_conn)?;
                let global_manager = GlobalManager::new(weak_app.clone_strong(), accounts);
                global_manager.set_accounts();
                Ok(())
            })();

            if let Err(e) = result {
                eprintln!("Error in add_account_handler: {}", e);
            }
        });
        Ok(())
    }
}