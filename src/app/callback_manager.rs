use slint::ComponentHandle;
use solana_sdk::msg;

pub struct CallbackManager<'a>  {
    app_instance: &'a crate::App
}

impl<'a> CallbackManager<'a> {
    pub fn new(app_instance: &'a crate::App) -> Self {
        CallbackManager { app_instance }
    }

    pub fn run(&self) {
        self.init_handlers();
    }

    fn init_handlers(&self) {
        self.view_account_handler();
    }

    fn view_account_handler(&self) {
        self.app_instance.global::<crate::AccountManager>().on_view_account(|pubkey| {
            let url = format!("https://solscan.io/account/{}", pubkey);

            if webbrowser::open(url.as_str()).is_ok() {
                msg!("Opened '{}' in your default web browser.", pubkey);
            } else {
                msg!("Failed to open '{}'.", pubkey);
            }
        });
    }

    fn add_account_handler(&self) {}
}