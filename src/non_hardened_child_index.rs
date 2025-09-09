use bc_envelope::prelude::*;

/// A non-hardened index used in hierarchical deterministic wallet derivation paths.
///
/// Non-hardened indices allow public key derivation, enabling watch-only wallets
/// to generate new addresses without having access to private keys. In BIP-44/ZIP-32
/// paths, the last two components (change and address_index) are typically non-hardened.
///
/// # Zcash Concept Relation
/// In Zcash HD wallet implementations:
/// - Hardened indices are shown with an apostrophe (e.g., `44'`)
/// - Non-hardened indices are shown without an apostrophe (e.g., `0` for external)
///
/// Non-hardened indices must be below 2^31 (0x80000000).
///
/// # Examples
/// ```
/// # use zewif::NonHardenedChildIndex;
/// // Create from a u32 value
/// let index = NonHardenedChildIndex::from(42u32);
///
/// // Convert back to u32 when needed
/// let value: u32 = index.into();
/// assert_eq!(value, 42);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NonHardenedChildIndex(u32);

/// Converts a u32 value to a NonHardenedChildIndex
impl From<u32> for NonHardenedChildIndex {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

/// Extracts the u32 value from a NonHardenedChildIndex
impl From<NonHardenedChildIndex> for u32 {
    fn from(value: NonHardenedChildIndex) -> Self {
        value.0
    }
}

/// Creates a NonHardenedChildIndex from a usize value (useful for array indexing)
impl From<usize> for NonHardenedChildIndex {
    fn from(value: usize) -> Self {
        Self(value as u32)
    }
}

impl From<NonHardenedChildIndex> for CBOR {
    fn from(value: NonHardenedChildIndex) -> Self {
        CBOR::from(value.0)
    }
}

impl From<&NonHardenedChildIndex> for CBOR {
    fn from(value: &NonHardenedChildIndex) -> Self {
        CBOR::from(value.0)
    }
}

impl TryFrom<CBOR> for NonHardenedChildIndex {
    type Error = dcbor::Error;

    fn try_from(value: CBOR) -> dcbor::Result<Self> {
        let position: u32 = value.try_into()?;
        Ok(NonHardenedChildIndex(position))
    }
}

impl From<NonHardenedChildIndex> for Envelope {
    fn from(value: NonHardenedChildIndex) -> Self {
        Envelope::new(CBOR::from(value))
    }
}

impl TryFrom<Envelope> for NonHardenedChildIndex {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.extract_subject()
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_cbor_roundtrip, test_envelope_roundtrip};

    use super::NonHardenedChildIndex;

    impl crate::RandomInstance for NonHardenedChildIndex {
        fn random() -> Self {
            Self(u32::random())
        }
    }

    test_cbor_roundtrip!(NonHardenedChildIndex);
    test_envelope_roundtrip!(NonHardenedChildIndex);
}
