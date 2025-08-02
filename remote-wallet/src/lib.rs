// remote-wallet/src/lib.rs
#![allow(dead_code)]

pub mod remote_wallet;
pub mod locator;
pub mod remote_keypair;
pub mod ledger;

pub use remote_wallet::{maybe_wallet_manager, RemoteWalletError, RemoteWalletManager};
pub use locator::{Locator, LocatorError};
pub use remote_keypair::generate_remote_keypair;