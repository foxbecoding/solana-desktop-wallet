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
    pub show_app_content: bool,
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
        let selected_account = self.get_selected_account()?;

        // used to set accounts vector
        // let rc_accounts: Rc<VecModel<Slintaccount>> = Rc::new(VecModel::from(accounts));
        // let model_rc_accounts = ModelRc::from(rc_accounts.clone());
        // crate::accountManager::get(&app).set_accounts(model_rc_accounts);

        crate::AccountManager::get(app).set_selected_account(selected_account);
        Ok(())
    }

    fn get_selected_account(&self) -> Result<SlintAccount, AppError> {
        match self.accounts.first() {
            Some(account) => {
                let account = SlintAccount {
                    id: account.id.unwrap(),
                    name: SharedString::from(account.name.clone()),
                    seed: SharedString::from(account.seed.clone()),
                    pubkey: SharedString::from(account.pubkey.clone()),
                    pubkey_display: account.pubkey_display(),
                    is_passphrase_protected: account.is_passphrase_protected,
                };
                Ok(account)
            },
            None => Err(AppError::NoAccountSelected),
        }
    }
}