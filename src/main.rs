// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use slint::PlatformError;

slint::include_modules!();
fn main() -> Result<(), PlatformError> {
    let app = App::new()?;
    // app.window().set_maximized(true);
    app.run()?;
    Ok(())
}
