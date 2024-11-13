use crate::database::account::Account;

pub struct GlobalManager<'a>  {
    app_instance: &'a crate::App,
    accounts: &'a Vec<Account>
}

impl<'a> GlobalManager<'a> {
    pub fn new(app_instance: &'a crate::App, accounts: &'a Vec<Account>) -> Self {
        GlobalManager { app_instance, accounts }
    }
}