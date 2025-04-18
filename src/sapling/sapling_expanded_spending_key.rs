use anyhow::{Context, Result};

use crate::{parse, test_envelope_roundtrip};
use super::super::parser::prelude::*;
use super::super::u256;
use bc_envelope::prelude::*;

/// Core cryptographic components of a Sapling spending key.
///
/// `SaplingExpandedSpendingKey` contains the three fundamental cryptographic components
/// that make up a Sapling spending key. These components collectively provide the ability
/// to spend funds, create nullifiers, and view outgoing transaction details in the
/// Sapling shielded protocol.
///
/// # Zcash Concept Relation
/// In Zcash's Sapling protocol, spending keys consist of several cryptographic components
/// that serve different purposes:
///
/// - **ask** (spending authorization key): A 256-bit scalar used to sign transactions,
///   authorizing the spending of funds
/// - **nsk** (nullifier private key): A 256-bit scalar used to create nullifiers for spent notes,
///   preventing double-spending
/// - **ovk** (outgoing viewing key): A 256-bit scalar used to view outgoing transaction details
///
/// Together, these components grant full control over Sapling shielded funds.
///
/// # Data Preservation
/// During wallet migration, all three key components must be preserved exactly to maintain
/// spending capability. These keys are never derived or recalculated - they are directly
/// stored in the wallet and must be transferred without modification during migration.
///
/// # Examples
/// ```
/// # use zewif::{sapling::SaplingExpandedSpendingKey, u256};
/// // Create an expanded spending key with the three components
/// let ask = u256::default(); // In practice, this would be a secure private key
/// let nsk = u256::default(); // In practice, this would be a secure private key
/// let ovk = u256::default(); // In practice, this would be a secure private key
///
/// let expsk = SaplingExpandedSpendingKey {
///     ask,
///     nsk,
///     ovk,
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SaplingExpandedSpendingKey {
    /// The spending authorization key, used to sign transactions
    pub ask: u256,
    /// The nullifier private key, used to create nullifiers for spent notes
    pub nsk: u256,
    /// The outgoing viewing key, used to view outgoing transaction details
    pub ovk: u256,
}

/// Implementation of the Parse trait for binary deserialization
impl Parse for SaplingExpandedSpendingKey {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(SaplingExpandedSpendingKey {
            ask: parse!(p, "ask")?,
            nsk: parse!(p, "nsk")?,
            ovk: parse!(p, "ovk")?,
        })
    }
}

impl From<SaplingExpandedSpendingKey> for Envelope {
    fn from(value: SaplingExpandedSpendingKey) -> Self {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(value.ask.as_ref());
        buffer.extend_from_slice(value.nsk.as_ref());
        buffer.extend_from_slice(value.ovk.as_ref());
        let cbor = CBOR::to_byte_string(&buffer);
        Envelope::new(cbor)
            .add_type("SaplingExpandedSpendingKey")
    }
}

impl TryFrom<Envelope> for SaplingExpandedSpendingKey {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        envelope.check_type_envelope("SaplingExpandedSpendingKey").context("SaplingExpandedSpendingKey")?;
        let bytes = envelope.subject().try_byte_string()?;
        parse!(buf = &bytes, SaplingExpandedSpendingKey, "SaplingExpandedSpendingKey")
    }
}

#[cfg(test)]
impl crate::RandomInstance for SaplingExpandedSpendingKey {
    fn random() -> Self {
        let ask = u256::random();
        let nsk = u256::random();
        let ovk = u256::random();
        Self { ask, nsk, ovk }
    }
}

test_envelope_roundtrip!(SaplingExpandedSpendingKey);
