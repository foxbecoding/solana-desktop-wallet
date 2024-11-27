use slint::{ComponentHandle};
use solana_sdk::msg;
use serde_json;
use crate::database::{cache::{Cache, CacheKey, CacheValue}, errors::DatabaseError, account::{Account, get_accounts}};
use crate::app::{global_manager::GlobalManager};
use crate::slint_generatedApp::{App as SlintApp, AccountManager, View as SlintViewEnum, ViewManager};

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
        self.change_account_handler()?;
        self.cache_active_view_handler()?;
        Ok(())
    }

    fn view_account_handler(&self) {
        self.app_instance.global::<AccountManager>().on_view_account(move |pubkey| {
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
        app.global::<AccountManager>().on_add_account(move || {
            let result = (|| -> Result<(), DatabaseError> {
                Account::new()?;
                let accounts = get_accounts()?;
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

    fn change_account_handler(&self) -> Result<(), DatabaseError> {
        let cache = Cache::new()?;
        self.app_instance.global::<AccountManager>().on_change_account(move |account_id| {
            let result = (|| -> Result<(), DatabaseError> {
                // Insert value into cache
                let cache_value = CacheValue {
                    value: account_id.to_string(),
                };
                cache.insert("selected_account", &cache_value)?;
                Ok(())
            })();

            if let Err(e) = result {
                eprintln!("Error in change_account_handler: {}", e);
            }
        });
        Ok(())
    }

    fn cache_active_view_handler(&self) -> Result<(), DatabaseError> {
        self.app_instance.global::<ViewManager>().on_cache_active_view(move |view: SlintViewEnum| {
            let result = (|| -> Result<(), DatabaseError> {
                let cache = Cache::new()?;
                let cache_key = CacheKey::SelectedView.key();
                let cache_value = CacheValue {
                    value: format!("{:?}", view),
                };
                cache.insert(&cache_key, &cache_value)?;
                Ok(())
            })();

            if let Err(e) = result {
                eprintln!("Error in on_cache_active_view: {}", e);
            }
        });
        Ok(())
    }
}