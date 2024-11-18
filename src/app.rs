pub(crate) mod errors;
pub(crate) mod callback_manager;
pub(crate) mod global_manager;

use slint::ComponentHandle;

use crate::database::account::Account;
use crate::app::{callback_manager::CallbackManager, global_manager::GlobalManager, errors::AppError};
#[derive(Debug)]
pub struct App {
    pub accounts: Vec<Account>
}

impl App {
    pub fn start(&self) -> Result<(), errors::AppError> {
        self.run_app()?;
        Ok(())
    }

    fn run_app(&self) -> Result<(), AppError> {
        let app = crate::App::new()?;
        self.run_managers(&app)?;
        app.run()?;
        Ok(())
    }

    fn run_managers(&self, app: &crate::App) -> Result<(), AppError> {
        global_manager::GlobalManager::new(&app, &self.accounts).run()?;
        callback_manager::CallbackManager::new(&app).run();
        Ok(())
    }
}