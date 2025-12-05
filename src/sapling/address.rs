use super::{
    SaplingExtendedFullViewingKey, SaplingExtendedSpendingKey,
    SaplingIncomingViewingKey,
};
use crate::{Blob, NoQuotesDebugOption, test_envelope_roundtrip};

use bc_envelope::prelude::*;

/// A Zcash Sapling address and associated key data.
///
/// Sapling addresses rely on zero-knowledge proofs to validate transactions without
/// revealing transaction details publicly. Each address can have associated keys:
///
/// - **Spending Keys**: Allow full control (viewing and spending)
/// - **Viewing Keys**: Allow monitoring transactions without spending capability
/// - **Diversifiers**: Enable generating multiple unique addresses from the same key material
///
/// # Data Preservation
/// During wallet migration, the following components are preserved:
///
/// - **Address strings**: The canonical string representation (e.g., "zs1...")
/// - **Incoming Viewing Keys (IVKs)**: Preserved for transaction monitoring capability
/// - **Spending Keys**: Preserved for spending capability when available
/// - **Diversifiers**: Preserved to maintain address derivation capability
/// - **HD paths**: Preserved to maintain hierarchical wallet structure
///
/// Note: Full Viewing Keys (FVKs) are not stored separately because they can be derived
/// from spending keys when needed, and source wallets typically don't store them separately.
///
/// # Examples
/// ```
/// # use zewif::{Blob, Data, sapling::{self, SaplingIncomingViewingKey, SaplingExtendedSpendingKey}};
/// // Create a new Sapling shielded address
/// let mut address = sapling::Address::new("zs1exampleaddress".to_string());
///
/// // Associate an incoming viewing key (for monitoring transactions)
/// let ivk = SaplingIncomingViewingKey::new([0u8; 32]);
/// address.set_incoming_viewing_key(ivk);
///
/// // For addresses with spending capability, add a spending key
/// let spending_key = SaplingExtendedSpendingKey::new([0u8; 169]);
/// address.set_spending_key(spending_key);
///
/// // Set the diversifier if available
/// let diversifier_index = Blob::new([0; 11]);
/// address.set_diversifier_index(diversifier_index);
///
/// // Set HD derivation path information
/// address.set_hd_derivation_path("m/32'/1'/0'/0/5".to_string());
/// ```
#[derive(Clone, PartialEq)]
pub struct Address {
    /// The string representaion of the address. This is used as a unique identifier within the
    /// wallet.
    address: String, // Unique

    /// Optional Incoming Viewing Key (IVK) for this address.
    ///
    /// When present, this 32-byte key allows the wallet to detect and view incoming transactions
    /// to this address without granting spending capability. This is particularly important for
    /// "watch-only" wallet functionality where spending keys aren't available.
    incoming_viewing_key: Option<SaplingIncomingViewingKey>,

    /// Optional Incoming Viewing Key (IVK) for this address.
    ///
    /// When present, this 32-byte key allows the wallet to detect and view transactions involving
    /// funds associated with this address without granting spending capability. This is
    /// particularly important for "watch-only" wallet functionality where spending keys aren't
    /// available.
    full_viewing_key: Option<SaplingExtendedFullViewingKey>,

    /// Optional spending key for this address.
    ///
    /// When present, this key allows spending funds sent to this address. During migration,
    /// spending keys are preserved exactly as they exist in the source wallet.
    spending_key: Option<SaplingExtendedSpendingKey>,

    /// HD derivation path if this address was derived using HD wallet techniques.
    ///
    /// This stores the path used to derive this address in a hierarchical deterministic wallet.
    /// Preserving this information allows wallets to reconstruct their address hierarchy.
    hd_derivation_path: Option<String>,

    /// The diversifier index used creating this address, if known, stored as a byte array in
    /// little-endian order.
    diversifier_index: Option<Blob<11>>,
}

impl std::fmt::Debug for Address {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SaplingAddress")
            .field("address", &self.address)
            .field(
                "incoming_viewing_key",
                &NoQuotesDebugOption(&self.incoming_viewing_key),
            )
            .field("spending_key", &self.spending_key)
            .field("diversifier_index", &self.diversifier_index)
            .field("hd_derivation_path", &self.hd_derivation_path)
            .finish()
    }
}

impl Address {
    pub fn new(address: String) -> Self {
        Address {
            address,
            incoming_viewing_key: None,
            full_viewing_key: None,
            spending_key: None,
            diversifier_index: None,
            hd_derivation_path: None,
        }
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn set_address(&mut self, address: String) {
        self.address = address;
    }

    /// Returns the Incoming Viewing Key (IVK) associated with this address, if available.
    ///
    /// The IVK enables viewing incoming transactions without granting spending capability.
    /// During wallet migration, IVKs are preserved exactly as they exist in the source wallet.
    ///
    /// # Returns
    /// - `Some(&SaplingIncomingViewingKey)` if an IVK is associated with this address
    /// - `None` if no IVK is available (common for addresses without viewing capability)
    pub fn incoming_viewing_key(&self) -> Option<&SaplingIncomingViewingKey> {
        self.incoming_viewing_key.as_ref()
    }

    /// Associates an Incoming Viewing Key (IVK) with this shielded address.
    ///
    /// This method is primarily used during wallet migration to preserve viewing capability
    /// for existing addresses. An IVK enables the wallet to scan the blockchain for incoming
    /// transactions without revealing transaction details publicly.
    ///
    /// # Parameters
    /// - `ivk`: The 32-byte Sapling Incoming Viewing Key to associate with this address
    pub fn set_incoming_viewing_key(&mut self, ivk: SaplingIncomingViewingKey) {
        self.incoming_viewing_key = Some(ivk);
    }

    pub fn full_viewing_key(&self) -> Option<&SaplingExtendedFullViewingKey> {
        self.full_viewing_key.as_ref()
    }

    pub fn set_full_viewing_key(&mut self, key: SaplingExtendedFullViewingKey) {
        self.full_viewing_key = Some(key);
    }

    pub fn spending_key(&self) -> Option<&SaplingExtendedSpendingKey> {
        self.spending_key.as_ref()
    }

    pub fn set_spending_key(&mut self, key: SaplingExtendedSpendingKey) {
        self.spending_key = Some(key);
    }

    pub fn diversifier_index(&self) -> Option<&Blob<11>> {
        self.diversifier_index.as_ref()
    }

    pub fn set_diversifier_index(&mut self, d: Blob<11>) {
        self.diversifier_index = Some(d);
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

impl From<Address> for Envelope {
    fn from(value: Address) -> Self {
        Envelope::new(value.address)
            .add_type("SaplingAddress")
            .add_optional_assertion(
                "incoming_viewing_key",
                value.incoming_viewing_key,
            )
            .add_optional_assertion("full_viewing_key", value.full_viewing_key)
            .add_optional_assertion("spending_key", value.spending_key)
            .add_optional_assertion(
                "diversifier_index",
                value.diversifier_index,
            )
            .add_optional_assertion(
                "hd_derivation_path",
                value.hd_derivation_path,
            )
    }
}

impl TryFrom<Envelope> for Address {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("SaplingAddress")?;
        let address = envelope.extract_subject()?;
        let incoming_viewing_key = envelope.try_optional_object_for_predicate("incoming_viewing_key")?;
        let full_viewing_key = envelope.try_optional_object_for_predicate("full_viewing_key")?;
        let spending_key = envelope.try_optional_object_for_predicate("spending_key")?;
        let diversifier_index = envelope.try_optional_object_for_predicate("diversifier_index")?;
        let hd_derivation_path = envelope.try_optional_object_for_predicate("hd_derivation_path")?;
        Ok(Address {
            address,
            incoming_viewing_key,
            full_viewing_key,
            spending_key,
            diversifier_index,
            hd_derivation_path,
        })
    }
}

#[cfg(test)]
impl crate::RandomInstance for Address {
    fn random() -> Self {
        Self {
            address: String::random(),
            incoming_viewing_key: SaplingIncomingViewingKey::opt_random(),
            full_viewing_key: SaplingExtendedFullViewingKey::opt_random(),
            spending_key: SaplingExtendedSpendingKey::opt_random(),
            diversifier_index: Blob::<11>::opt_random(),
            hd_derivation_path: String::opt_random(),
        }
    }
}

test_envelope_roundtrip!(Address);
