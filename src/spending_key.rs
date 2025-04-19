use super::Blob;
use crate::sapling::SaplingExtendedSpendingKey;
use crate::test_envelope_roundtrip;
use anyhow::{Context, bail};
use bc_envelope::prelude::*;

/// A spending key that provides full control over funds, enabling transaction creation
/// and authorization for a specific address in a Zcash wallet.
///
/// `SpendingKey` is a protocol-agnostic wrapper for various Zcash spending key types.
/// It contains the cryptographic material necessary to spend funds and view all transaction
/// details associated with an address, making it the most sensitive component of a wallet.
///
/// # Zcash Concept Relation
/// In Zcash, spending keys represent the highest level of control over funds:
///
/// - **Sapling extended spending keys**: Used for Sapling shielded addresses (zs-prefixed)
///
/// The key hierarchy in Zcash allows deriving less-privileged keys from spending keys:
/// ```text
/// Spending Key → Full Viewing Key → Incoming Viewing Key
/// (full control)   (can view all)    (can only view incoming)
/// ```
///
/// # Data Preservation
/// During wallet migration, spending keys are preserved exactly as they exist in the
/// source wallet to maintain complete control over funds. Each variant preserves the
/// appropriate protocol-specific key material.
#[derive(Clone, Debug, PartialEq)]
pub enum SpendingKey {
    /// Sapling protocol spending key with full cryptographic components
    Sapling(SaplingExtendedSpendingKey),
}

impl From<SpendingKey> for Envelope {
    fn from(value: SpendingKey) -> Self {
        match value {
            SpendingKey::Sapling(sapling_spending_key) => {
                Envelope::new(CBOR::from(sapling_spending_key.to_vec()))
                    .add_type("SaplingExtendedSpendingKey")
            }
        }
        .add_type("SpendingKey")
    }
}

impl TryFrom<Envelope> for SpendingKey {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        envelope
            .check_type_envelope("SpendingKey")
            .context("SpendingKey")?;
        let subject = envelope.subject();
        if subject.has_type_envelope("SaplingExtendedSpendingKey") {
            let extsk =
                SaplingExtendedSpendingKey::from_vec(Blob::<169>::try_from(subject)?.to_vec())?;
            Ok(SpendingKey::Sapling(extsk))
        } else {
            bail!("Invalid SpendingKey envelope: {}", envelope.format());
        }
    }
}

#[cfg(test)]
impl crate::RandomInstance for SpendingKey {
    fn random() -> Self {
        Self::Sapling(SaplingExtendedSpendingKey::from_vec(Blob::<169>::random().to_vec()).unwrap())
    }
}

test_envelope_roundtrip!(SpendingKey);
