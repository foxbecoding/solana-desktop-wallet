use std::rc::Rc;
use slint::{Global, ModelRc, SharedString, VecModel};
use crate::app::errors::AppError;
use crate::database::account::Account;
use crate::slint_generatedApp::Account as SlintAccount;

pub struct GlobalManager<'a>  {
    app_instance: &'a crate::App,
    accounts: &'a Vec<Account>
}

impl<'a> GlobalManager<'a> {
    pub fn new(app_instance: &'a crate::App, accounts: &'a Vec<Account>) -> Self {
        GlobalManager { app_instance, accounts }
    }

    pub fn init_globals(&self) -> Result<(), AppError> {
        self.set_selected_account()?;
        self.set_accounts();
        Ok(())
    }

    fn set_selected_account(&self) -> Result<(), AppError> {
        match self.accounts.first() {
            Some(account) => {
                let slint_account = slint_account_builder(account);
                crate::AccountManager::get(self.app_instance).set_selected_account(slint_account);
                Ok(())
            },
            None => Err(AppError::NoAccountSelected),
        }
    }

    fn set_accounts(&self) {
        let mut slint_accounts: Vec<SlintAccount> = vec!();
        for account in self.accounts.clone() {
            let slint_account = slint_account_builder(&account);
            slint_accounts.push(slint_account);
        }

        let rc_accounts: Rc<VecModel<SlintAccount>> = Rc::new(VecModel::from(slint_accounts));
        let model_rc_accounts = ModelRc::from(rc_accounts.clone());
        crate::AccountManager::get(self.app_instance).set_accounts(model_rc_accounts);
    }
}

fn slint_account_builder(account: &Account) -> SlintAccount{
    SlintAccount {
        id: account.id.unwrap(),
        name: SharedString::from(account.name.clone()),
        seed: SharedString::from(account.seed.clone()),
        pubkey: SharedString::from(account.pubkey.clone()),
        pubkey_display: account.pubkey_display(),
    }
}