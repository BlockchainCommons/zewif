use crate::{UnifiedAddress, error::Error, sapling, transparent};
use bc_envelope::prelude::*;

/// A protocol-specific Zcash address representation without additional metadata.
///
/// `ProtocolAddress` is an enum that distinguishes between the different address
/// protocols supported in Zcash. It provides a type-safe way to handle the varied
/// address formats while enabling common operations across all address types.
///
/// # Zcash Concept Relation
/// Zcash has evolved through multiple address formats, each with different privacy
/// and functionality characteristics:
///
/// - **Transparent addresses (t-prefixed)**: Function like Bitcoin addresses,
///   exposing all transaction details on the blockchain.
///
/// - **Shielded addresses (z-prefixed)**: Use zero-knowledge proofs to encrypt
///   transaction details. Originally included Sprout (legacy), now primarily
///   Sapling (zs-prefixed) and Orchard (zo-prefixed) protocols.
///
/// - **Unified addresses (u-prefixed)**: Introduced in NU5, these bundle multiple
///   receiver types into a single address, allowing the sender's wallet to automatically
///   choose the most private protocol supported by both parties.
///
/// # Data Preservation
/// During wallet migration, the complete address details from each protocol are preserved:
///
/// - **For transparent addresses**: Base58Check-encoded address strings and associated keys
/// - **For shielded addresses**: Encoded address strings, diversifiers, and viewing keys
/// - **For unified addresses**: All component addresses, diversifier indices, and metadata
///
/// # Examples
/// ```
/// # use zewif::{ProtocolAddress, transparent, sapling, UnifiedAddress};
/// // Create a transparent address
/// let t_addr = transparent::Address::new("t1example");
/// let t_protocol = ProtocolAddress::Transparent(t_addr);
/// assert!(t_protocol.is_transparent());
///
/// // Create a Sapling address
/// let s_addr = sapling::Address::new("zs1example".to_string());
/// let s_protocol = ProtocolAddress::Sapling(Box::new(s_addr));
/// assert!(s_protocol.is_sapling());
///
/// // Create a unified address
/// let u_addr = UnifiedAddress::new("u1example".to_string());
/// let u_protocol = ProtocolAddress::Unified(Box::new(u_addr));
/// assert!(u_protocol.is_unified());
///
/// // All protocol addresses can be converted to strings
/// assert!(t_protocol.as_string().starts_with("t1"));
/// assert!(s_protocol.as_string().starts_with("zs1"));
/// assert!(u_protocol.as_string().starts_with("u1"));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum ProtocolAddress {
    /// An exposed transparent (T-address) similar to Bitcoin's.
    Transparent(transparent::Address),

    /// A Sapling address (Z-address).
    Sapling(Box<sapling::Address>),

    /// A unified address (U-address) that contains multiple receiver types.
    Unified(Box<UnifiedAddress>),
}

impl ProtocolAddress {
    /// Returns the address as a string in its canonical format.
    ///
    /// This method returns the string representation of the address, regardless
    /// of which protocol it uses. This is useful for display or storage where
    /// only the address string is needed.
    ///
    /// # Returns
    /// A string representation of the address.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{ProtocolAddress, transparent, sapling, UnifiedAddress};
    /// #
    /// // Transparent address
    /// let t_addr = transparent::Address::new("t1example");
    /// let protocol = ProtocolAddress::Transparent(t_addr);
    /// assert_eq!(protocol.as_string(), "t1example");
    ///
    /// // Shielded address
    /// let s_addr = sapling::Address::new("zs1example".to_string());
    /// let protocol = ProtocolAddress::Sapling(Box::new(s_addr));
    /// assert_eq!(protocol.as_string(), "zs1example");
    /// ```
    pub fn as_string(&self) -> String {
        match self {
            ProtocolAddress::Transparent(addr) => addr.address().to_string(),
            ProtocolAddress::Sapling(addr) => addr.address().to_string(),
            ProtocolAddress::Unified(addr) => addr.address().to_string(),
        }
    }

    /// Returns true if this is a Sapling address.
    ///
    /// # Returns
    /// `true` if the address is a Sapling address (z-address), `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{Address, ProtocolAddress, sapling, transparent};
    /// #
    /// // Create a Sapling address
    /// let s_addr = sapling::Address::new("zs1example".to_string());
    /// let address = ProtocolAddress::Sapling(Box::new(s_addr));
    /// assert!(address.is_sapling());
    ///
    /// // Create a transparent address
    /// let t_addr = transparent::Address::new("t1example");
    /// let address = ProtocolAddress::Transparent(t_addr);
    /// assert!(!address.is_sapling());
    /// ```
    pub fn is_sapling(&self) -> bool {
        matches!(self, ProtocolAddress::Sapling(_))
    }

    /// Returns true if this is a transparent address.
    ///
    /// # Returns
    /// `true` if the address is a transparent address (t-address), `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{Address, ProtocolAddress, sapling, transparent};
    /// #
    /// // Create a transparent address
    /// let t_addr = transparent::Address::new("t1example");
    /// let address = ProtocolAddress::Transparent(t_addr);
    /// assert!(address.is_transparent());
    ///
    /// // Create a shielded address
    /// let s_addr = sapling::Address::new("zs1example".to_string());
    /// let address = ProtocolAddress::Sapling(Box::new(s_addr));
    /// assert!(!address.is_transparent());
    /// ```
    pub fn is_transparent(&self) -> bool {
        matches!(self, ProtocolAddress::Transparent(_))
    }

    /// Returns true if this is a unified address.
    ///
    /// # Returns
    /// `true` if the address is a unified address (u-address), `false` otherwise.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{Address, ProtocolAddress, UnifiedAddress, transparent};
    /// #
    /// // Create a unified address
    /// let u_addr = UnifiedAddress::new("u1example".to_string());
    /// let address = ProtocolAddress::Unified(Box::new(u_addr));
    /// assert!(address.is_unified());
    ///
    /// // Create a transparent address
    /// let t_addr = transparent::Address::new("t1example");
    /// let address = ProtocolAddress::Transparent(t_addr);
    /// assert!(!address.is_unified());
    /// ```
    pub fn is_unified(&self) -> bool {
        matches!(self, ProtocolAddress::Unified(_))
    }
}

impl From<ProtocolAddress> for Envelope {
    fn from(value: ProtocolAddress) -> Self {
        match value {
            ProtocolAddress::Transparent(addr) => addr.into(),
            ProtocolAddress::Sapling(addr) => (*addr).into(),
            ProtocolAddress::Unified(addr) => (*addr).into(),
        }
    }
}

impl TryFrom<Envelope> for ProtocolAddress {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        if envelope.has_type("TransparentAddress") {
            Ok(ProtocolAddress::Transparent(envelope.try_into()?))
        } else if envelope.has_type("SaplingAddress") {
            Ok(ProtocolAddress::Sapling(Box::new(envelope.try_into()?)))
        } else if envelope.has_type("UnifiedAddress") {
            Ok(ProtocolAddress::Unified(Box::new(envelope.try_into()?)))
        } else {
            Err(Error::InvalidProtocolAddress.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ProtocolAddress;
    use crate::{UnifiedAddress, sapling, test_envelope_roundtrip, transparent};

    impl crate::RandomInstance for ProtocolAddress {
        fn random() -> Self {
            let mut rng = rand::rng();
            let choice = rand::Rng::random_range(&mut rng, 0..3);
            match choice {
                0 => ProtocolAddress::Transparent(transparent::Address::random()),
                1 => ProtocolAddress::Sapling(Box::new(sapling::Address::random())),
                _ => ProtocolAddress::Unified(Box::new(UnifiedAddress::random())),
            }
        }
    }

    test_envelope_roundtrip!(ProtocolAddress);
}
