pub(crate) mod errors;
pub(crate) mod callback_manager;
pub(crate) mod global_manager;
use slint::ComponentHandle;
use std::sync::{Arc, Mutex};
use crate::database::account::Account;

#[derive(Debug)]
pub struct App {
    pub accounts: Vec<Account>
}

impl App {
    pub fn start(&self) -> Result<(), errors::AppError> {
        self.run_app()?;
        Ok(())
    }

    fn run_app(&self) -> Result<(), errors::AppError> {
        let app = crate::App::new()?;

        // Wrapping the mutable reference in Arc and Mutex
        let arc_app_instance = Arc::new(Mutex::new(app));

        self.run_managers(Arc::clone(&arc_app_instance))?;
        arc_app_instance.lock().unwrap().run()?;
        Ok(())
    }

    fn run_managers(&self, arc_app_instance: Arc<Mutex<crate::App>>) -> Result<(), errors::AppError> {
        global_manager::GlobalManager::new(Arc::clone(&arc_app_instance), &self.accounts).run()?;
        callback_manager::CallbackManager::new(Arc::clone(&arc_app_instance)).run()?;
        Ok(())
    }
}