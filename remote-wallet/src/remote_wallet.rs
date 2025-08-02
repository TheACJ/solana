use std::{error::Error, fmt, rc::Rc};

use crate::ledger::LedgerWallet;

use crate::locator::Locator;

#[derive(Debug)]
pub struct RemoteWallet {
    pub ledger: LedgerWallet,
}

impl RemoteWallet {
    pub fn get_pubkey(
        &self,
        path: &solana_sdk::derivation_path::DerivationPath,
        confirm: bool,
    ) -> Result<solana_sdk::pubkey::Pubkey, RemoteWalletError> {
        self.ledger.get_pubkey(path, confirm)
    }

    pub fn sign_message(
        &self,
        path: &solana_sdk::derivation_path::DerivationPath,
        message: &[u8],
    ) -> Result<solana_sdk::signature::Signature, RemoteWalletError> {
        self.ledger.sign_message(path, message)
    }
}


#[derive(Debug)]
pub struct RemoteWalletInfo {
    pub manufacturer: crate::locator::Manufacturer,
}

impl RemoteWalletInfo {
    pub fn parse_locator(locator: Locator) -> Self {
        Self {
            manufacturer: locator.manufacturer,
        }
    }
}

#[derive(Debug)]
pub enum RemoteWalletType {
    Ledger(RemoteWallet),
}
#[derive(Debug, Clone)]
pub struct RemoteWalletManager;

impl RemoteWalletManager {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {})
    }
}

// Stubbed `maybe_wallet_manager` returns what clap-utils expects
pub fn maybe_wallet_manager() -> Result<Rc<RemoteWalletManager>, Box<dyn Error>> {
    Ok(Rc::new(RemoteWalletManager::new()?))
}

#[derive(Debug)]
pub enum RemoteWalletError {
    DeviceTypeMismatch,
    Other(String),
    NoDeviceFound,
}

/*#[derive(Debug, Error)]
pub enum RemoteWalletError {
    #[error("Device type mismatch")]
    DeviceTypeMismatch,

    #[error("No remote wallet device found")]
    

    // You can add others as needed, depending on future errors
}
*/
impl fmt::Display for RemoteWalletError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RemoteWalletError::DeviceTypeMismatch => write!(f, "Device type mismatch"),
            RemoteWalletError::Other(msg) => write!(f, "Remote wallet error: {msg}"),
            RemoteWalletError::NoDeviceFound => write!(f, "No remote wallet device found"),
        }
    }
}

impl Error for RemoteWalletError {}