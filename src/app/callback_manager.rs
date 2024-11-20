use slint::ComponentHandle;
use solana_sdk::msg;
use std::sync::{Arc, Mutex};
use crate::database::{database_connection, errors::DatabaseError, account::{Account, get_accounts, insert_account}};
use crate::app::global_manager::GlobalManager;

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
        app_instance.lock().unwrap().global::<crate::AccountManager>().on_add_account({
            let app_instance = Arc::clone(&app_instance);
            let db_conn = database_connection()?;
            move || {
                let result = (|| -> Result<(), DatabaseError> {
                    // Establish db connection

                    // get accounts count
                    let accounts_count = get_accounts(&db_conn)?.len();
                    // set new account name
                    let new_account_name = format!("Account {}", accounts_count + 1);
                    // insert into DB
                    let new_account = Account::new(&db_conn, new_account_name)?;
                    insert_account(&db_conn, &new_account)?;
                    // set accounts in app
                    let accounts = get_accounts(&db_conn)?;
                    let global_manager = GlobalManager::new(Arc::clone(&app_instance), &accounts);
                    global_manager.set_accounts();
                    Ok(())
                })();

                if let Err(e) = result {
                    eprintln!("Error in add_account_handler: {}", e);
                }
            }
        });
        Ok(())
    }
}