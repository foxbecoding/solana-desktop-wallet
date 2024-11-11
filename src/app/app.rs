use crate::database::{
    database_connection,
    errors::DatabaseError,
    account::{Account, insert_account},
};

use std::rc::Rc;
use slint::{Global, ComponentHandle, ModelRc, SharedString, VecModel};

use crate::app::errors::AppError;
use crate::slint_generatedApp::Account as SlintAccount;

#[derive(Debug)]
pub struct App {
    pub accounts: Vec<Account>,
    pub show_app_content: bool,
}

impl App {
    pub fn start(&self) -> Result<(), AppError> {
        self.run_app()?;
        Ok(())
    }

    fn run_app(&self) -> Result<(), AppError> {
        let app = crate::App::new()?;

        //for testing
        // let pubkey = Pubkey::new_unique();
        //
        // let wallet =
        //     SlintWallet {
        //         id: 1,
        //         name: SharedString::from("Main Account".to_string()),
        //         seed: SharedString::from("Some Seed Phrase".to_string()),
        //         public_key: SharedString::from(pubkey.to_string()),
        //         public_key_display: pubkey_display_generator(pubkey.to_string()),
        //         is_passphrase_protected: false,
        //     };
        //
        // let wallets = vec![wallet.clone()];
        //
        //
        // // used to set wallets vector
        // let rc_wallets: Rc<VecModel<SlintWallet>> = Rc::new(VecModel::from(wallets));
        // let model_rc_wallets = ModelRc::from(rc_wallets.clone());
        // crate::WalletManager::get(&app).set_wallets(model_rc_wallets);
        //
        // crate::WalletManager::get(&app).set_selected_wallet(wallet);

        app.run()?;
        Ok(())
    }
}