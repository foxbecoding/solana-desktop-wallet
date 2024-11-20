use slint::ComponentHandle;
use solana_sdk::msg;
use std::sync::{Arc, Mutex};
use crate::database::{database_connection, errors::DatabaseError, account::{Account, get_accounts, insert_account}};

pub struct CallbackManager {
    app_instance: Arc<Mutex<crate::App>>,
}

impl CallbackManager {
    pub fn new(app_instance: Arc<Mutex<crate::App>>) -> Self {
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
        let app_instance = Arc::clone(&self.app_instance);
        app_instance.lock().unwrap().global::<crate::AccountManager>().on_view_account(move |pubkey| {
            let url = format!("https://solscan.io/account/{}", pubkey);

            if webbrowser::open(url.as_str()).is_ok() {
                msg!("Opened '{}' in your default web browser.", pubkey);
            } else {
                msg!("Failed to open '{}'.", pubkey);
            }
        });
    }

    fn add_account_handler(&self) -> Result<(), DatabaseError> {
        let app_instance = Arc::clone(&self.app_instance);
        app_instance.lock().unwrap().global::<crate::AccountManager>().on_add_account(move || {
            let app_instance = Arc::clone(&app_instance);
            if let Err(e) = (|| -> Result<(), DatabaseError> {
                // Establish db connection
                let db_conn = database_connection()?;
                // get accounts count
                let accounts_count = get_accounts(&db_conn)?.len();
                // set new account name
                let new_account_name = format!("Account {}", accounts_count + 1);
                // let new_account = Account::new(&db_conn, new_account_name)?;
                // insert_account(&db_conn, &new_account)?;
                println!("{}", new_account_name);

                Ok(())
            })() {
                eprintln!("Error in add_account_handler: {}", e);
            }
        });
        Ok(())
    }
}