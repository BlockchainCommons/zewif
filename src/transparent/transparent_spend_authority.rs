use super::TransparentSpendingKey;
use bc_envelope::prelude::*;

/// The cryptographic authorization needed to spend funds from a transparent Zcash address.
///
/// `TransparentSpendAuthority` represents the spending capability for transparent
/// addresses (those starting with 't'). It distinguishes between directly stored keys
/// and keys that are derived from another source, such as an HD wallet seed.
///
/// # Zcash Concept Relation
/// In Zcash's transparent address system (inherited from Bitcoin):
///
/// - Spending requires proving ownership through cryptographic signatures
/// - This typically involves a private key corresponding to a public key hash (P2PKH)
///   or a script hash (P2SH)
/// - In hierarchical deterministic (HD) wallets, transparent keys are often derived
///   from a master seed using BIP-44 paths
///
/// # Data Preservation
/// During wallet migration, the `TransparentSpendAuthority` preserves:
///
/// - Directly stored spending keys that exist in the source wallet
/// - Information about keys that are derived from HD wallet seeds
///
/// This ensures that spending capability is maintained after migration while
/// preserving the wallet's key management structure.
///
/// # Examples
/// ```
/// # use zewif::{Blob, transparent::{TransparentSpendAuthority, TransparentSpendingKey}};
/// // Direct spending key
/// let spending_key = TransparentSpendingKey::new([0; 32]);
/// let spend_authority = TransparentSpendAuthority::SpendingKey(spending_key);
///
/// // Derived key (from HD wallet seed)
/// let derived_authority = TransparentSpendAuthority::Derived;
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum TransparentSpendAuthority {
    /// Direct spending key stored in the wallet
    SpendingKey(TransparentSpendingKey),

    /// Spending key derived from another source (e.g., HD wallet seed)
    /// The actual derivation information is typically stored with the address
    Derived,
}

impl From<TransparentSpendAuthority> for Envelope {
    fn from(value: TransparentSpendAuthority) -> Self {
        match value {
            TransparentSpendAuthority::SpendingKey(key) => key.into(),
            TransparentSpendAuthority::Derived => Envelope::new("Derived"),
        }
        .add_type("TransparentSpendAuthority")
    }
}

impl TryFrom<Envelope> for TransparentSpendAuthority {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("TransparentSpendAuthority")?;
        if let Ok(spending_key) = TransparentSpendingKey::try_from(envelope.clone()) {
            Ok(TransparentSpendAuthority::SpendingKey(spending_key))
        } else if envelope.extract_subject::<String>()? == "Derived" {
            Ok(TransparentSpendAuthority::Derived)
        } else {
            Err(crate::error::Error::InvalidTransparentSpendAuthority.into())
        }
    }
}

#[cfg(test)]
impl crate::RandomInstance for TransparentSpendAuthority {
    fn random() -> Self {
        let mut rng = rand::thread_rng();
        let a = rand::Rng::gen_range(&mut rng, 0..=1);
        if a == 0 {
            TransparentSpendAuthority::SpendingKey(TransparentSpendingKey::random())
        } else {
            TransparentSpendAuthority::Derived
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_envelope_roundtrip;

    use super::TransparentSpendAuthority;

    test_envelope_roundtrip!(TransparentSpendAuthority);
}
