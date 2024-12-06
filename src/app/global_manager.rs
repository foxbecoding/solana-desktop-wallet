use crate::app::{app_view_selector, errors::AppError};
use crate::database::{
    account::Account,
    cache::{fetch_cache_value, Cache, CacheKey},
};
use crate::slint_generatedApp::{
    Account as SlintAccount, AccountManager, App as SlintApp, ViewManager,
};
use slint::{Global, ModelRc, SharedString, VecModel};
use std::rc::Rc;

pub struct GlobalManager {
    app_instance: SlintApp,
    accounts: Vec<Account>,
}

impl GlobalManager {
    pub fn new(app_instance: SlintApp, accounts: Vec<Account>) -> Self {
        GlobalManager {
            app_instance,
            accounts,
        }
    }

    pub fn run(&self) -> Result<(), AppError> {
        self.init_globals()?;
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

    fn init_globals(&self) -> Result<(), AppError> {
        self.set_selected_account()?;
        self.set_accounts();
        self.set_selected_view()?;
        Ok(())
    }

    fn set_selected_account(&self) -> Result<(), AppError> {
        // Set first account by default
        let mut account = self.accounts.first();

        let conn = self.conn.clone();
        let cache = Cache::new(conn);

        // Check cache for selected account
        if let Some(selected_account_id) = fetch_cache_value(&cache, &CacheKey::SelectedAccount)? {
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
        let cache = Cache::new()?;
        if let Some(selected_view) = fetch_cache_value(&cache, &CacheKey::SelectedView)? {
            let view = app_view_selector(selected_view);
            ViewManager::get(&self.app_instance).set_active_view(view);
        }
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
