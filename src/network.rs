use std::str::FromStr;

use anyhow::{Context, Result, anyhow};
use bc_envelope::prelude::*;

/// Represents a Zcash network environment (mainnet, testnet, or regtest).
///
/// The `Network` enum identifies which Zcash network a wallet, address,
/// or transaction belongs to. Each network has different consensus rules,
/// address encodings, and initial blockchain parameters.
///
/// # Zcash Concept Relation
/// Zcash, like Bitcoin, operates on multiple networks:
///
/// - **Mainnet**: The primary Zcash network where real ZEC with monetary value is transferred
/// - **Testnet**: A testing network that simulates mainnet but uses worthless test coins
/// - **Regtest**: A private "regression test" network for local development and testing
///
/// These networks are isolated from each other, with different genesis blocks,
/// address formats, and consensus parameters.
///
/// # Data Preservation
/// The `Network` value is critical during wallet migration to ensure addresses and
/// transactions are reconstructed for the correct network. Address formats differ
/// between networks, and migrating a wallet to an incorrect network would render
/// it unusable.
///
/// # Examples
/// In the ZeWIF format, the Network value is stored at the wallet level:
/// ```
/// # use zewif::{ZewifWallet, Network};
/// // Wallet on the main Zcash network
/// let network = Network::Main;
///
/// // Wallets on mainnet and testnet have incompatible address formats
/// match network {
///     Network::Main => println!("This wallet stores real ZEC"),
///     Network::Test => println!("This wallet stores test coins only"),
///     Network::Regtest => println!("This wallet is for local testing"),
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Network {
    Main,
    Test,
    Regtest,
}

impl Network {
    fn encode(&self) -> &'static str {
        match self {
            Network::Main => "main",
            Network::Test => "test",
            Network::Regtest => "regtest",
        }
    }

    fn decode(value: &str) -> Option<Self> {
        match value {
            "main" => Some(Network::Main),
            "test" => Some(Network::Test),
            "regtest" => Some(Network::Regtest),
            _ => None,
        }
    }
}

impl core::fmt::Display for Network {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.encode())
    }
}

impl FromStr for Network {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Self::decode(s).ok_or(anyhow!("Invalid network identifier: {}", s))
    }
}

impl From<Network> for String {
    fn from(value: Network) -> String {
        value.encode().to_string()
    }
}

impl TryFrom<String> for Network {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::decode(&value).ok_or(anyhow!("Invalid network identifier: {}", value))
    }
}

impl From<Network> for CBOR {
    fn from(value: Network) -> Self {
        String::from(value).into()
    }
}

impl TryFrom<CBOR> for Network {
    type Error = dcbor::Error;

    fn try_from(cbor: CBOR) -> dcbor::Result<Self> {
        Ok(cbor.try_into_text()?.try_into()?)
    }
}

impl From<Network> for Envelope {
    fn from(value: Network) -> Self {
        Envelope::new(String::from(value))
    }
}

impl TryFrom<Envelope> for Network {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        let network_str: String = envelope.extract_subject().context("Network")?;
        Network::try_from(network_str)
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_cbor_roundtrip, test_envelope_roundtrip};

    use super::Network;

    impl crate::RandomInstance for Network {
        fn random() -> Self {
            match rand::random::<u8>() % 3 {
                0 => Network::Main,
                1 => Network::Test,
                _ => Network::Regtest,
            }
        }
    }

    test_cbor_roundtrip!(Network);
    test_envelope_roundtrip!(Network);
}
