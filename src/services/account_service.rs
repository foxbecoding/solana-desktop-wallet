use rusqlite::Connection;
use std::sync::{Arc, Mutex};

use crate::database::{
    account::{get_accounts, Account},
    errors::DatabaseError,
};

pub struct AccountService {
    conn: Arc<Mutex<Connection>>,
}

impl AccountService {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }
}
