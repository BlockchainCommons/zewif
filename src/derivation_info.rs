use bc_envelope::prelude::*;

use crate::NonHardenedChildIndex;

/// Hierarchical deterministic (HD) derivation information for wallet addresses.
///
/// `DerivationInfo` captures the BIP-44/ZIP-32 derivation path components for
/// addresses in a hierarchical deterministic wallet. It specifically tracks the
/// last two non-hardened components of an HD path:
/// - Whether it's a change address (typically 0 = external, 1 = change)
/// - The address index within that chain
///
/// # Zcash Concept Relation
/// Zcash follows BIP-44 and ZIP-32 for hierarchical deterministic key derivation,
/// with paths typically structured as:
/// ```text
/// m / purpose' / coin_type' / account' / change / address_index
/// ```
///
/// Where:
/// - `purpose'` is typically 44' for transparent or 32' for shielded
/// - `coin_type'` is 133' for Zcash
/// - `account'` is the account number (hardened)
/// - `change` is 0 for external addresses or 1 for internal (change) addresses
/// - `address_index` is the sequential index of the address
///
/// The apostrophes (') indicate hardened derivation, which prevents parent key
/// compromise from affecting child keys.
///
/// # Data Preservation
/// During wallet migration, this information is preserved to maintain the
/// hierarchical relationship between keys and the ability to derive the same
/// addresses in the new wallet.
///
/// # Examples
/// ```
/// # use zewif::{DerivationInfo, NonHardenedChildIndex};
/// // Create derivation info for an external address (change = 0)
/// // with index 5
/// let change = NonHardenedChildIndex::from(0u32); // external
/// let address_index = NonHardenedChildIndex::from(5u32);
///
/// // We need a constructor to create DerivationInfo
/// let derivation_info = DerivationInfo::new(change, address_index);
///
/// // The values can be retrieved for further derivation or reference
/// assert_eq!(u32::from(derivation_info.change()), 0);
/// assert_eq!(u32::from(derivation_info.address_index()), 5);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DerivationInfo {
    /// The change level (0 = external addresses, 1 = internal/change addresses)
    change: NonHardenedChildIndex,

    /// The address index at the specified change level
    address_index: NonHardenedChildIndex,
}

impl DerivationInfo {
    /// Creates a new `DerivationInfo` with the specified change and address index components.
    ///
    /// # Arguments
    /// * `change` - The change level index (0 for external, 1 for internal/change)
    /// * `address_index` - The sequential index of the address within its chain
    ///
    /// # Examples
    /// ```
    /// # use zewif::{DerivationInfo, NonHardenedChildIndex};
    /// // Create derivation info for an external address (change = 0)
    /// // with index 5
    /// let change = NonHardenedChildIndex::from(0u32);
    /// let address_index = NonHardenedChildIndex::from(5u32);
    /// let derivation_info = DerivationInfo::new(change, address_index);
    /// ```
    pub fn new(change: NonHardenedChildIndex, address_index: NonHardenedChildIndex) -> Self {
        Self {
            change,
            address_index,
        }
    }

    /// Returns the change component of the derivation path.
    ///
    /// In BIP-44/ZIP-32, the change component is:
    /// - 0 for external (receiving) addresses
    /// - 1 for internal (change) addresses
    ///
    /// # Returns
    /// The change index as a `NonHardenedChildIndex`
    pub fn change(&self) -> NonHardenedChildIndex {
        self.change
    }

    /// Returns the address index component of the derivation path.
    ///
    /// This is the sequential index of an address within its chain
    /// (external or change). Each new address in a wallet typically
    /// increments this index.
    ///
    /// # Returns
    /// The address index as a `NonHardenedChildIndex`
    pub fn address_index(&self) -> NonHardenedChildIndex {
        self.address_index
    }
}

impl From<DerivationInfo> for Envelope {
    fn from(value: DerivationInfo) -> Self {
        Envelope::new(value.change)
            .add_type("DerivationInfo")
            .add_assertion("address_index", value.address_index)
    }
}

impl TryFrom<Envelope> for DerivationInfo {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type_envelope("DerivationInfo")?;
        let change = envelope.extract_subject()?;
        let address_index = envelope.extract_object_for_predicate("address_index")?;
        Ok(Self {
            change,
            address_index,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{NonHardenedChildIndex, test_envelope_roundtrip};

    use super::DerivationInfo;

    impl crate::RandomInstance for DerivationInfo {
        fn random() -> Self {
            Self {
                change: NonHardenedChildIndex::random(),
                address_index: NonHardenedChildIndex::random(),
            }
        }
    }

    test_envelope_roundtrip!(DerivationInfo);
}
