use std::rc::Rc;
use slint::{Global, ComponentHandle, ModelRc, SharedString, VecModel};
use webbrowser;

use crate::database::{
    account::{Account},
};
use crate::app::{callback_manager::CallbackManager, errors::AppError};
use crate::slint_generatedApp::Account as SlintAccount;


#[derive(Debug)]
pub struct App {
    pub accounts: Vec<Account>
}

impl App {
    pub fn start(&self) -> Result<(), AppError> {
        self.run_app()?;
        Ok(())
    }

    fn run_app(&self) -> Result<(), AppError> {
        let app = crate::App::new()?;
        self.set_app_globals(&app)?;
        CallbackManager::new(&app).run();
        app.run()?;
        Ok(())
    }

    fn set_app_globals(&self, app: &crate::App) -> Result<(), AppError> {
        self.set_selected_account(app)?;
        self.set_accounts_global(app);
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

    fn set_accounts_global(&self, app: &crate::App)  {
        let mut slint_accounts: Vec<SlintAccount> = vec!();
        for account in self.accounts.clone() {
            let slint_account = slint_account_builder(&account);
            slint_accounts.push(slint_account);
        }

        let rc_accounts: Rc<VecModel<SlintAccount>> = Rc::new(VecModel::from(slint_accounts));
        let model_rc_accounts = ModelRc::from(rc_accounts.clone());
        crate::AccountManager::get(app).set_accounts(model_rc_accounts);
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