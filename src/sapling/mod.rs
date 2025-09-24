//! # Sapling Protocol Components
//!
//! The Sapling protocol is the second-generation shielded protocol in Zcash,
//! introduced in the Sapling network upgrade (October 2018). It significantly
//! improved upon the original Sprout protocol with better performance, security
//! features, and usability.
//!
//! This module contains types that represent various components of the Sapling
//! protocol within the ZeWIF wallet interchange format, including:
//!
//! ## Key Components
//!
//! - [`SaplingExpandedSpendingKey`]: Core cryptographic components of a Sapling
//!   spending key (ask, nsk, ovk)
//! - [`SaplingExtendedSpendingKey`]: Hierarchical deterministic key structure
//!   for Sapling according to ZIP-32
//! - [`SaplingIncomingViewingKey`]: Key for detecting and viewing incoming
//!   transactions only
//! - [`SaplingSpendingKey`]: Spending authority for Sapling addresses
//!
//! ## Transaction Components
//!
//! - [`SaplingWitness`]: Cryptographic witness proving a note commitment exists
//!   in the tree
//! - [`SaplingSentOutput`]: Sender's record of note data for outgoing
//!   transactions
//!
//! ## Protocol Characteristics
//!
//! Sapling introduced significant improvements over the earlier Sprout
//! protocol:
//!
//! * **Performance**: Much faster proving times (~7 seconds vs ~40 seconds in
//!   Sprout)
//! * **Key Separation**: Full viewing keys that don't reveal spending
//!   capability
//! * **HD Wallet Support**: ZIP-32 hierarchical deterministic key derivation
//! * **Decoupled Spend/Output**: Separated spend and output descriptions
//!   (unlike Sprout's JoinSplits)
//!
//! These types collectively enable the migration of Sapling shielded data
//! between different Zcash wallet implementations while preserving all
//! cryptographic capabilities.

use crate::mod_use;

mod_use!(address);
mod_use!(sapling_extended_spending_key);
mod_use!(sapling_extended_full_viewing_key);
mod_use!(sapling_incoming_viewing_key);
mod_use!(sapling_sent_output);
mod_use!(sapling_witness);
