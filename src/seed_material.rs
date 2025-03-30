use super::Blob;

/// Source material used to generate cryptographic keys in a Zcash wallet.
///
/// `SeedMaterial` represents the fundamental secret data from which a wallet derives
/// all its cryptographic keys. It can be either a BIP-39 mnemonic phrase (seed words)
/// or a raw 32-byte seed that predates the BIP-39 standard.
///
/// # Zcash Concept Relation
/// In Zcash wallet systems:
///
/// - **BIP-39 Mnemonics**: Human-readable seed phrases (typically 12 or 24 words) that
///   encode entropy in a memorable format. These phrases are used to generate HD wallet
///   hierarchical deterministic keys following BIP-32/BIP-44 derivation paths.
///
/// - **Pre-BIP39 Seeds**: Raw 32-byte seeds from older wallet implementations that
///   predate the BIP-39 standard. These are typically stored as binary data and lack
///   the mnemonic recovery mechanism.
///
/// Wallet implementations use this seed material as the root of their key derivation,
/// generating both transparent and shielded keys from this source.
///
/// # Data Preservation
/// During wallet migration, the seed material is the most critical component to preserve:
///
/// - **Mnemonic Phrases**: The complete, unmodified word sequence in the correct order
/// - **Raw Seeds**: The exact 32-byte value without any modification
///
/// Preserving this data ensures a wallet can be reconstructed with all its derivable keys
/// and addresses intact, providing access to all funds.
///
/// # Security Considerations
/// Seed material is extremely sensitive information that provides complete control over
/// all wallet funds. It must be handled with appropriate security precautions:
///
/// - Never transmit unencrypted over networks
/// - Store encrypted at rest
/// - Ensure secure memory handling to prevent leaks
///
/// # Examples
/// ```
/// use zewif::{SeedMaterial, Blob};
///
/// // Create from a BIP-39 mnemonic phrase
/// let mnemonic = SeedMaterial::Bip39Mnemonic(
///     "abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon abandon about"
///     .to_string()
/// );
///
/// // Create from a pre-BIP39 raw seed
/// let raw_seed = [0u8; 32];
/// let seed_blob = Blob::new(raw_seed);
/// let binary_seed = SeedMaterial::PreBIP39Seed(seed_blob);
/// ```
#[derive(Clone)]
pub enum SeedMaterial {
    /// A BIP-39 mnemonic phrase (typically 12 or 24 words) used as a human-readable seed
    Bip39Mnemonic(String),
    /// A raw 32-byte seed predating the BIP-39 standard
    PreBIP39Seed(Blob<32>),
}

impl std::fmt::Debug for SeedMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bip39Mnemonic(phrase) => write!(f, "SeedMaterial::Bip39Mnemonic(\"{}\")", phrase),
            Self::PreBIP39Seed(seed) => write!(f, "SeedMaterial::PreBIP39Seed({:?})", seed),
        }
    }
}

impl std::fmt::Display for SeedMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bip39Mnemonic(phrase) => write!(f, "SeedMaterial::Bip39Mnemonic(\"{}\")", phrase),
            Self::PreBIP39Seed(seed) => write!(f, "SeedMaterial::PreBIP39Seed({:?})", seed),
        }
    }
}
