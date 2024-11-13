use crate::app::callback_manager::CallbackManager;
use crate::database::account::Account;

pub struct GlobalManager<'a>  {
    app_instance: &'a crate::App,
    accounts: Vec<Account>
}

impl<'a> CallbackManager<'a> {}