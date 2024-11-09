// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod database;

use slint::{include_modules as include_slint_modules};

include_slint_modules!();
fn main() -> Result<(), app::AppError> {
    app::App::start()?;
    Ok(())
}
