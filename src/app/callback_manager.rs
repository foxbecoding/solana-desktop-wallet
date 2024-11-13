struct CallbackManager<'a>  {
    app_instance: &'a crate::App
}

impl CallbackManager {
    fn new(app_instance: &crate::App) -> Self {
        CallbackManager { app_instance }
    }
}