use crate::database::account::Account;

pub struct GlobalManager<'a>  {
    accounts: Vec<Account>
}