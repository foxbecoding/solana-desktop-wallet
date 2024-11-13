struct CallbackManager {
    app_instance: crate::App
}

impl CallbackManager {
    fn new(app_instance: crate::App) -> Self {
        CallbackManager { app_instance }
    }
}