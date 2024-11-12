use crate::database::{
    account::{Account},
};

use std::rc::Rc;
use slint::{Global, ComponentHandle, ModelRc, SharedString, VecModel};

use crate::app::errors::AppError;
use crate::slint_generatedApp::Account as SlintAccount;

#[derive(Debug)]
pub struct App {
    pub accounts: Vec<Account>,
    pub show_app_view: bool,
}

impl App {
    pub fn start(&self) -> Result<(), AppError> {
        self.run_app()?;
        Ok(())
    }

    fn run_app(&self) -> Result<(), AppError> {
        let app = crate::App::new()?;
        self.set_app_globals(&app)?;
        app.run()?;
        Ok(())
    }

    fn set_app_globals(&self, app: &crate::App) -> Result<(), AppError> {
        self.set_selected_account(app)?;
        let accounts = self.get_accounts();

        // used to set accounts vector
        let rc_accounts: Rc<VecModel<SlintAccount>> = Rc::new(VecModel::from(accounts));
        let model_rc_accounts = ModelRc::from(rc_accounts.clone());
        crate::AccountManager::get(&app).set_accounts(model_rc_accounts);

        Ok(())
    }

    fn set_selected_account(&self, app: &crate::App) -> Result<(), AppError> {
       match self.accounts.first() {
            Some(account) => {
                let slint_account = slint_account_builder(account);
                crate::AccountManager::get(app).set_selected_account(slint_account);
                Ok(())
            },
            None => Err(AppError::NoAccountSelected),
        }

    }

    fn get_accounts(&self) -> Vec<SlintAccount> {
        let mut slint_accounts: Vec<SlintAccount> = vec!();
        for account in self.accounts.clone() {
            let slint_account = slint_account_builder(&account);
            slint_accounts.push(slint_account);
        }
        slint_accounts
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