use crate::database::account::Account;

pub struct GlobalManager<'a>  {
    app_instance: &'a crate::App,
    accounts: Vec<Account>
}