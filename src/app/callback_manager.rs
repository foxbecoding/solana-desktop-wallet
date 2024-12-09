use crate::app::global_manager::GlobalManager;
use crate::database::{
    cache::{Cache, CacheKey, CacheValue},
    errors::DatabaseError,
};
use crate::services::account_service::AccountService;
use crate::slint_generatedApp::{
    AccountManager, App as SlintApp, View as SlintViewEnum, ViewManager,
};
use rusqlite::Connection;
use slint::ComponentHandle;
use solana_sdk::msg;
use std::sync::{Arc, Mutex};

pub struct CallbackManager {
    app_instance: SlintApp,
    conn: Arc<Mutex<Connection>>,
}

impl CallbackManager {
    pub fn new(conn: Arc<Mutex<Connection>>, app_instance: SlintApp) -> Self {
        CallbackManager { app_instance, conn }
    }

    pub fn run(&self) -> Result<(), DatabaseError> {
        self.init_handlers()?;
        Ok(())
    }

    fn init_handlers(&self) -> Result<(), DatabaseError> {
        self.view_account_handler();
        self.add_account_handler()?;
        self.change_account_handler()?;
        self.cache_active_view_handler()?;
        Ok(())
    }

    fn view_account_handler(&self) {
        self.app_instance
            .global::<AccountManager>()
            .on_view_account(move |pubkey| {
                let url = format!("https://solscan.io/account/{}", pubkey);

                if webbrowser::open(url.as_str()).is_ok() {
                    msg!("Opened '{}' in your default web browser.", pubkey);
                } else {
                    msg!("Failed to open '{}'.", pubkey);
                }
            });
    }

    fn add_account_handler(&self) -> Result<(), DatabaseError> {
        let conn = self.conn.clone();
        let app = self.app_instance.clone_strong();
        let weak_app = app.as_weak().unwrap();
        app.global::<AccountManager>().on_add_account(move || {
            let conn = conn.clone();
            let result = (|| -> Result<(), DatabaseError> {
                let account_service = AccountService::new(conn.clone());
                let accounts = account_service.get_all_accounts()?;
                let global_manager = GlobalManager::new(conn, weak_app.clone_strong(), accounts);
                global_manager.set_accounts();
                Ok(())
            })();

            if let Err(e) = result {
                eprintln!("Error in add_account_handler: {}", e);
            }
        });
        Ok(())
    }

    fn change_account_handler(&self) -> Result<(), DatabaseError> {
        let conn = self.conn.clone();
        let cache = Cache::new(conn);
        self.app_instance
            .global::<AccountManager>()
            .on_change_account(move |account_id| {
                let result = (|| -> Result<(), DatabaseError> {
                    let cache_value = CacheValue {
                        value: account_id.to_string(),
                    };
                    cache.set(&CacheKey::SelectedAccount, &cache_value)?;
                    Ok(())
                })();

                if let Err(e) = result {
                    eprintln!("Error in change_account_handler: {}", e);
                }
            });
        Ok(())
    }

    fn cache_active_view_handler(&self) -> Result<(), DatabaseError> {
        let conn = self.conn.clone();
        let cache = Cache::new(conn);
        self.app_instance
            .global::<ViewManager>()
            .on_cache_active_view(move |view: SlintViewEnum| {
                let result = (|| -> Result<(), DatabaseError> {
                    let cache_value = CacheValue {
                        value: format!("{:?}", view),
                    };
                    cache.set(&CacheKey::SelectedView, &cache_value)?;
                    Ok(())
                })();

                if let Err(e) = result {
                    eprintln!("Error in on_cache_active_view: {}", e);
                }
            });
        Ok(())
    }
}
