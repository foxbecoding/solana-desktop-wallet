// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use  std::error::Error;

slint::include_modules!();
fn main() -> Result<(), Box<dyn Error>> {
    let app = App::new()?;

    app.on_request_increase_value({
        let app_handle = app.as_weak();
        move || {
            let app = app_handle.unwrap();
            app.set_counter(app.get_counter() + 3);
        }
    });

    Ok(app.run()?)
}
