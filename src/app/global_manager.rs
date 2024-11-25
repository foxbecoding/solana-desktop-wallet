use std::rc::Rc;
use slint::{Global, ModelRc, SharedString, VecModel};
use crate::app::errors::AppError;
use crate::database::{cache::Cache, account::Account};
use crate::slint_generatedApp::{App as SlintApp, Account as SlintAccount, AccountManager};

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

    fn init_globals(&self) -> Result<(), AppError> {
        self.set_selected_account()?;
        self.set_accounts();
        Ok(())
    }

    fn set_selected_account(&self) -> Result<(), AppError> {
        let mut account = self.accounts.first();

        //Check cache first
        let cache = Cache::new()?;
        if let Some(value) = cache.get("selected_account")? {
            let value = value.value;
            for acc in  self.accounts.iter() {
                if acc.id.unwrap().to_string() == value {
                    account = Some(acc);
                }
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