// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::include_modules as include_slint_modules;

mod app;
mod connection;
mod database;
mod initializer;

use crate::app::errors::AppError;
use crate::database::database_connection;
use crate::initializer::run as Run_Initializer;

include_slint_modules!();
fn main() -> Result<(), AppError> {
    let conn = database_connection()?;
    Run_Initializer(&conn)?;
    Ok(())
}
