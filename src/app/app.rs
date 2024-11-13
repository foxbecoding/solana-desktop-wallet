use slint::{ComponentHandle, SharedString};
use webbrowser;

use crate::database::{
    account::{Account},
};
use crate::app::{callback_manager::CallbackManager, global_manager::GlobalManager, errors::AppError};
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
        self.init_managers(&app)?;
        app.run()?;
        Ok(())
    }

    fn init_managers(&self, app: &crate::App) -> Result<(), AppError> {
        GlobalManager::new(&app, &self.accounts).run()?;
        CallbackManager::new(&app).run();
        Ok(())
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