use crate::remote_wallet::{RemoteWallet, RemoteWalletError, RemoteWalletInfo, RemoteWalletManager};

#[derive(Debug)]
pub struct LedgerWallet {
    pub pretty_path: String,
}

impl LedgerWallet {
    pub fn get_pubkey(
        &self,
        _path: &solana_sdk::derivation_path::DerivationPath,
        _confirm: bool,
    ) -> Result<solana_sdk::pubkey::Pubkey, RemoteWalletError> {
        Ok(solana_sdk::pubkey::Pubkey::new_unique())
    }

    pub fn sign_message(
        &self,
        _path: &solana_sdk::derivation_path::DerivationPath,
        _message: &[u8],
    ) -> Result<solana_sdk::signature::Signature, RemoteWalletError> {
        Ok(solana_sdk::signature::Signature::default())
    }
}

pub fn get_ledger_from_info(
    _info: RemoteWalletInfo,
    _keypair_name: &str,
    _manager: &RemoteWalletManager,
) -> Result<RemoteWallet, RemoteWalletError> {
    Ok(RemoteWallet {
        ledger: LedgerWallet {
            pretty_path: "usb://ledger/".to_string(),
        },
    })
}