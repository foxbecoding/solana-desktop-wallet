use crate::app::{app_view_selector, errors::AppError};
use crate::database::{account::Account, cache::Cache};
use crate::slint_generatedApp::{
    Account as SlintAccount, AccountManager, App as SlintApp, SolValueManager, ViewManager,
};
use crate::token_value::TokenValue;
use anyhow::Error as AnyhowError;
use jupiter_api::client::Client as JupiterClient;
use rusqlite::Connection;
use slint::{Global, ModelRc, SharedString, VecModel};
use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

pub struct GlobalManager {
    app_instance: SlintApp,
    accounts: Vec<Account>,
    conn: Arc<Mutex<Connection>>,
}

impl GlobalManager {
    pub fn new(
        conn: Arc<Mutex<Connection>>,
        app_instance: SlintApp,
        accounts: Vec<Account>,
    ) -> Self {
        GlobalManager {
            app_instance,
            accounts,
            conn,
        }
    }

    pub async fn run(&self) -> Result<(), AppError> {
        self.init_globals().await?;
        Ok(())
    }

    async fn init_globals(&self) -> Result<(), AppError> {
        self.set_selected_account()?;
        self.set_accounts();
        self.set_selected_view()?;
        self.set_sol_usd_value().await?;
        Ok(())
    }

    pub fn set_accounts(&self) {
        let mut slint_accounts: Vec<SlintAccount> = vec![];
        for account in self.accounts.clone() {
            let slint_account = slint_account_builder(&account);
            slint_accounts.push(slint_account);
        }

        let rc_accounts: Rc<VecModel<SlintAccount>> = Rc::new(VecModel::from(slint_accounts));
        let model_rc_accounts = ModelRc::from(rc_accounts.clone());
        AccountManager::get(&self.app_instance).set_accounts(model_rc_accounts);
    }

    fn set_selected_account(&self) -> Result<(), AppError> {
        // Set first account by default
        let mut account = self.accounts.first();

        let conn = self.conn.clone();
        let cache = Cache::new(conn);

        // Check cache for selected account
        if let Some(selected_account_id) = cache.get_selected_account()? {
            if let Some(acc) = self.find_account_by_id(&selected_account_id) {
                account = Some(acc);
            }
        }

        match account {
            Some(account) => {
                let slint_account = slint_account_builder(account);
                AccountManager::get(&self.app_instance).set_selected_account(slint_account);
                Ok(())
            }
            None => Err(AppError::NoAccountSelected),
        }
    }

    fn find_account_by_id(&self, id: &str) -> Option<&Account> {
        self.accounts
            .iter()
            .find(|acc| acc.id.unwrap().to_string() == id)
    }

    fn set_selected_view(&self) -> Result<(), AppError> {
        let conn = self.conn.clone();
        let cache = Cache::new(conn);

        if let Some(selected_view) = cache.get_selected_view()? {
            let view = app_view_selector(selected_view);
            ViewManager::get(&self.app_instance).set_active_view(view);
        }
        Ok(())
    }

    async fn set_sol_usd_value(&self) -> Result<(), AppError> {
        const SOL_KEY: &str = "So11111111111111111111111111111111111111112";
        let token_keys = [SOL_KEY];

        // Create a TokenValue instance and fetch prices
        let token_value = TokenValue::new(&token_keys).await?;

        // Print out the prices for each key
        for id in &token_value.ids {
            if let Some(data) = token_value.get_price(id) {
                println!(
                    "ID: {}, Type: {}, Price: {}",
                    data.id, data.token_type, data.price
                );
            } else {
                println!("No data found for the key: {}", id);
            }
        }
        //SolValueManager::get(&self.app_instance).set_value(valug);
        Ok(())
    }
}

fn slint_account_builder(account: &Account) -> SlintAccount {
    SlintAccount {
        id: account.id.unwrap(),
        name: SharedString::from(account.name.clone()),
        seed: SharedString::from(account.seed.clone()),
        pubkey: SharedString::from(account.pubkey.clone()),
        pubkey_display: account.pubkey_display(),
        balance: account.balance_in_sol() as f32,
    }
}
