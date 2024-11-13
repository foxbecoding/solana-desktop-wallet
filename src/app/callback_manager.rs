use slint::ComponentHandle;
use solana_sdk::msg;

struct CallbackManager<'a>  {
    app_instance: &'a crate::App
}

impl CallbackManager {
    pub fn new(app_instance: &crate::App) -> Self {
        CallbackManager { app_instance }
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
}