use crate::database::account::Account;

pub struct GlobalManager<'a>  {
    app_instance: &'a crate::App,
    accounts: Vec<Account>
}

impl<'a> GlobalManager<'a> {
    pub fn new(app_instance: &'a crate::App) {
    }
}