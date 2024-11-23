use slint::ComponentHandle;

pub(crate) mod errors;
pub(crate) mod callback_manager;
pub(crate) mod global_manager;
use crate::database::account::Account;
use crate::app::errors::AppError;
use crate::slint_generatedApp::{App as SlintApp};

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
        let app = SlintApp::new()?;
        let weak_app = app.as_weak().unwrap();
        self.run_managers(weak_app)?;
        app.run()?;
        Ok(())
    }

    fn run_managers(&self, app_instance: crate::App) -> Result<(), AppError> {
        global_manager::GlobalManager::new(app_instance.clone_strong(), self.accounts.clone()).run()?;
        callback_manager::CallbackManager::new(app_instance).run()?;
        Ok(())
    }
}