use std::rc::Rc;
use slint::{Global, ModelRc, SharedString, VecModel};
use crate::app::{app_view_selector, errors::AppError};
use crate::database::{cache::{Cache, CacheKey}, account::Account};
use crate::slint_generatedApp::{
    App as SlintApp, Account as SlintAccount,
    AccountManager, ViewManager
};

pub struct GlobalManager {
    app_instance: SlintApp,
    accounts: Vec<Account>
}

impl GlobalManager {
    pub fn new(app_instance: SlintApp, accounts: Vec<Account>) -> Self {
        GlobalManager { app_instance, accounts }
    }

    pub fn run(&self) -> Result<(), AppError> {
        self.init_globals()?;
        Ok(())
    }

    pub fn set_accounts(&self) {
        let mut slint_accounts: Vec<SlintAccount> = vec!();
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
        // Initialize first account by default
        let mut account = self.accounts.first();

        // Check cache for selected account
        if let Some(selected_account_id) = self.get_selected_account_from_cache()? {
            if let Some(acc) = self.find_account_by_id(&selected_account_id) {
                account = Some(acc);
            }
        }

        match account {
            Some(account) => {
                let slint_account = slint_account_builder(account);
                AccountManager::get(&self.app_instance).set_selected_account(slint_account);
                Ok(())
            },
            None => Err(AppError::NoAccountSelected),
        }
    }

    fn get_selected_account_from_cache(&self) -> Result<Option<String>, AppError> {
        let cache = Cache::new()?;
        if let Some(value) = cache.get(&CacheKey::SelectedAccount)? {
            Ok(Some(value.value))
        } else {
            Ok(None)
        }
    }

    fn find_account_by_id(&self, id: &str) -> Option<&Account> {
        self.accounts.iter().find(|acc| acc.id.unwrap().to_string() == id)
    }

    fn set_selected_view(&self) -> Result<(), AppError> {
        if let Some(selected_view) = self.get_selected_view_from_cache()? {
            let view = app_view_selector(selected_view);
            ViewManager::get(&self.app_instance).set_active_view(view);
        }
        Ok(())
    }

    fn get_selected_view_from_cache(&self) -> Result<Option<String>, AppError> {
        let cache = Cache::new()?;
        if let Some(value) = cache.get(&CacheKey::SelectedView)? {
            Ok(Some(value.value))
        } else {
            Ok(None)
        }
    }
}

fn slint_account_builder(account: &Account) -> SlintAccount{
    SlintAccount {
        id: account.id.unwrap(),
        name: SharedString::from(account.name.clone()),
        seed: SharedString::from(account.seed.clone()),
        pubkey: SharedString::from(account.pubkey.clone()),
        pubkey_display: account.pubkey_display(),
        balance: account.balance_in_sol() as f32
    }
}