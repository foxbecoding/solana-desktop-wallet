use slint::{ComponentHandle, PlatformError};

pub struct App {}

impl App {
    pub fn start() -> Result<(), PlatformError> {
        let app = crate::App::new()?;
        app.run()?;
        Ok(())
    }
}