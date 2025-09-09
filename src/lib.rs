//! # Zcash Wallet Interchange Format (ZeWIF)
//!
//! `zewif` is a library that defines a standard data format for migrating wallet data
//! between different Zcash wallet implementations. It provides a comprehensive set of
//! types, tools, and utilities for serializing, deserializing, and manipulating Zcash
//! wallet data in a way that preserves all critical information during migration.
//!
//! ## Core Features
//!
//! * **Complete Wallet Data Model**: Represents all aspects of a Zcash wallet including
//!   accounts, addresses, transactions, and keys
//! * **Multi-Protocol Support**: Handles the Transparent, Sapling, and Orchard Zcash protocols.
//! * **Type-Safe Representation**: Uses Rust's type system to ensure correct handling of Zcash concepts
//! * **Extensible Metadata**: Supports custom metadata through an attachments system
//!
//! ## Core Components
//!
//! The ZeWIF format is organized hierarchically:
//!
//! - [`Zewif`]: The root container holding wallets and global transaction data
//!   - [`ZewifWallet`]: Individual wallet with accounts and network context
//!     - [`Account`]: Logical grouping of addresses and transaction references
//!       - [`Address`]: Individual addresses of various types (transparent, shielded, unified)
//!   - [`Transaction`]: Complete transaction data (inputs, outputs, metadata)
//!
//! ## Protocol Support
//!
//! ZeWIF handles the Zcash protocol versions:
//!
//! - **Transparent**: Bitcoin-compatible public transactions ([`TransparentAddress`], [`TxIn`], [`TxOut`])
//! - **Sapling**: Improved shielded protocol ([`sapling`] module, [`sapling::SaplingSentOutput`], etc.)
//! - **Orchard**: Latest shielded protocol ([`orchard`] module, [`orchard::OrchardSentOutput`], etc.)
//!
//! ## Integration Path
//!
//! This crate is part of a larger ecosystem:
//!
//! - `zewif`: Core library defining the interchange format (this crate)
//! - `zmigrate`: Command-line tool for wallet migrations
//! - `zewif-zcashd`: ZCashd-specific integration for migration
//! - `zewif-zingo`: Zingo-specific integration for migration (future)
//!
//! ## Usage Examples
//!
//! ```no_run
//! use zewif::{Zewif, ZewifWallet, Network, Account, Address, BlockHeight};
//!
//! // Create a new ZeWIF container
//! let mut zewif = Zewif::new(BlockHeight::from_u32(2000000));
//!
//! // Create a new wallet for the main network
//! let mut wallet = ZewifWallet::new(Network::Main);
//!
//! // Add a new account to the wallet
//! let mut account = Account::new();
//! account.set_name("Default Account");
//!
//! // Add the account to the wallet and the wallet to the ZeWIF container
//! wallet.add_account(account);
//! zewif.add_wallet(wallet);
//! ```

// Macros
mod blob_macro;
mod data_macro;
mod envelope_macros;
mod mod_use_macro;
mod string_macro;
mod test_roundtrip_macros;

// Test utilities
#[cfg(any(test, feature = "test-dependencies"))]
mod_use!(test_utils);

// Modules requiring qualified paths
pub mod orchard;
pub mod sapling;
pub mod transparent;

// Modules that can use unqualified paths
mod_use!(account);
mod_use!(address);
mod_use!(amount);
mod_use!(anchor);
mod_use!(bip_39_mnemonic);
mod_use!(blob);
mod_use!(block_hash);
mod_use!(block_height);
mod_use!(data);
mod_use!(error);
mod_use!(derivation_info);
mod_use!(incremental_witness);
mod_use!(indexed);
mod_use!(memo);
mod_use!(mnemonic_language);
mod_use!(network);
mod_use!(non_hardened_child_index);
mod_use!(protocol_address);
mod_use!(script);
mod_use!(legacy_seed);
mod_use!(seed_material);
mod_use!(seed_fingerprint);
mod_use!(string_utils);
mod_use!(transaction);
mod_use!(tx_block_position);
mod_use!(txid);
mod_use!(unified_address);
mod_use!(zewif_envelope);
mod_use!(zewif_impl);
mod_use!(zewif_wallet);

pub use blob::Blob;
use std::fmt::{self, Debug, Display, Formatter};

#[doc(hidden)]
pub struct NoQuotesDebugOption<'a, T>(pub &'a Option<T>);

impl<T: Display> Debug for NoQuotesDebugOption<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(val) => write!(f, "Some({})", val),
            None => write!(f, "None"),
        }
    }
}

#[doc(hidden)]
pub struct DebugOption<'a, T>(&'a Option<T>);

impl<T: Debug> Debug for DebugOption<'_, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.0 {
            Some(val) => write!(f, "Some({:?})", val),
            None => write!(f, "None"),
        }
    }
}
