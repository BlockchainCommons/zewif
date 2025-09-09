use crate::Blob;
use bc_envelope::prelude::*;

/// A multi-protocol Zcash address that can contain components from different Zcash protocols.
///
/// `UnifiedAddress` represents Zcash's next-generation addressing format that allows bundling
/// multiple receiver types (transparent, Sapling, Orchard) into a single encoded string.
/// This enables wallets to generate one address that can receive funds via any supported
/// protocol while hiding the underlying complexity from users.
///
/// # Zcash Concept Relation
/// Unified Addresses (UAs) were introduced in ZIP-316 and deployed with NU5 (Orchard
/// network upgrade). They address several limitations of individual address types:
///
/// - **Simplified User Experience**: Users see a single address (starting with "u1...")
///   rather than dealing with multiple address types
/// - **Forward Compatibility**: New receiver types can be added without breaking existing UAs
/// - **Protocol Agnostic**: Senders don't need to choose which protocol to use; their wallet
///   can select the most appropriate one based on capability
/// - **Progressive Privacy**: Transactions automatically use the best privacy technology
///   available to both sender and receiver
///
/// Each UA contains one or more "receivers" from different protocols (transparent, Sapling,
/// and/or Orchard). When funds are sent to a UA, the sender's wallet selects the most
/// private protocol that both wallets support.
///
/// # Data Preservation
/// During wallet migration, the following components are preserved:
///
/// - **UA string**: The full unified address string when available
/// - **Diversifier index**: Used to deterministically derive addresses from keys
/// - **HD path information**: For addresses derived from hierarchical deterministic wallets
///
/// # Implementation Note
/// In Zcash wallets, unified addresses are not typically stored directly in wallet.dat files.
/// Instead, wallets store the metadata needed to derive UAs at runtime:
/// - Diversifier indices
/// - Information about which receiver types to include
/// - References to the keys used to derive the addresses
///
/// # Examples
/// ```
/// # use zewif::{UnifiedAddress, Blob};
/// // Create a new unified address
/// let mut ua = UnifiedAddress::new("u1exampleaddress".to_string());
///
/// // Set a diversifier index (used in address derivation)
/// let diversifier_data = [0u8; 11];
/// let diversifier_index = Blob::new(diversifier_data);
/// ua.set_diversifier_index(diversifier_index);
#[derive(Clone, PartialEq)]
pub struct UnifiedAddress {
    /// The full unified address string (starting with "u...")
    address: String,

    /// The diversifier index used to derive this unified address.
    /// This is an 11-byte unsigned integer in little-endian order.
    diversifier_index: Option<Blob<11>>,

    /// HD derivation path if this address was derived using HD wallet techniques
    hd_derivation_path: Option<String>,
}

impl std::fmt::Debug for UnifiedAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UnifiedAddress")
            .field("address", &self.address)
            .field("diversifier_index", &self.diversifier_index)
            .field("hd_derivation_path", &self.hd_derivation_path)
            .finish()
    }
}

impl UnifiedAddress {
    /// Create a new UnifiedAddress with the given address string
    pub fn new(address: String) -> Self {
        UnifiedAddress {
            address,
            diversifier_index: None,
            hd_derivation_path: None,
        }
    }

    /// Creates a new UnifiedAddress from its constituent parts.
    pub fn from_parts(
        address: String,
        diversifier_index: Option<Blob<11>>,
        hd_derivation_path: Option<String>,
    ) -> Self {
        UnifiedAddress {
            address,
            diversifier_index,
            hd_derivation_path,
        }
    }

    /// Get the full unified address string
    pub fn address(&self) -> &str {
        &self.address
    }

    /// Set the unified address string
    pub fn set_address(&mut self, address: String) {
        self.address = address;
    }

    /// Get the diversifier index if available
    pub fn diversifier_index(&self) -> Option<&Blob<11>> {
        self.diversifier_index.as_ref()
    }

    /// Set the diversifier index
    pub fn set_diversifier_index(&mut self, diversifier_index: Blob<11>) {
        self.diversifier_index = Some(diversifier_index);
    }

    /// Get the HD derivation path for this address, if available
    pub fn hd_derivation_path(&self) -> Option<&str> {
        self.hd_derivation_path.as_deref()
    }

    /// Set the HD derivation path for this address
    pub fn set_hd_derivation_path(&mut self, path: String) {
        self.hd_derivation_path = Some(path);
    }
}

impl From<UnifiedAddress> for Envelope {
    fn from(value: UnifiedAddress) -> Self {
        Envelope::new(value.address)
            .add_type("UnifiedAddress")
            .add_optional_assertion("diversifier_index", value.diversifier_index)
            .add_optional_assertion("hd_derivation_path", value.hd_derivation_path)
    }
}

impl TryFrom<Envelope> for UnifiedAddress {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type_envelope("UnifiedAddress")?;
        let address = envelope.extract_subject()?;
        let diversifier_index = envelope.try_optional_object_for_predicate("diversifier_index")?;
        let hd_derivation_path = envelope.try_optional_object_for_predicate("hd_derivation_path")?;

        Ok(UnifiedAddress {
            address,
            diversifier_index,
            hd_derivation_path,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Blob, test_envelope_roundtrip};

    use super::UnifiedAddress;

    impl crate::RandomInstance for UnifiedAddress {
        fn random() -> Self {
            Self {
                address: String::random(),
                diversifier_index: Blob::opt_random(),
                hd_derivation_path: String::opt_random(),
            }
        }
    }

    test_envelope_roundtrip!(UnifiedAddress);
}
