use slint::{Global, ComponentHandle, ModelRc, SharedString, VecModel};
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

    fn init_globals(&self) -> Result<(), AppError> {
        self.set_selected_account()?;
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

    fn set_accounts_global(&self) {

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