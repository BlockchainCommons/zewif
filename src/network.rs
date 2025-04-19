use anyhow::{Context, Result, bail};
use bc_envelope::prelude::*;
use zcash_protocol::consensus::NetworkType;

use crate::{test_cbor_roundtrip, test_envelope_roundtrip};

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
pub struct Network(NetworkType);

#[allow(non_upper_case_globals)]
impl Network {
    pub const Main: Self = Network(NetworkType::Main);
    pub const Test: Self = Network(NetworkType::Test);
    pub const Regtest: Self = Network(NetworkType::Regtest);
}

impl From<NetworkType> for Network {
    fn from(value: NetworkType) -> Self {
        Network(value)
    }
}

impl From<Network> for NetworkType {
    fn from(value: Network) -> Self {
        value.0
    }
}

impl From<Network> for String {
    fn from(value: Network) -> String {
        match value.0 {
            NetworkType::Main => "main".to_string(),
            NetworkType::Test => "test".to_string(),
            NetworkType::Regtest => "regtest".to_string(),
        }
    }
}

impl TryFrom<String> for Network {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value == "main" {
            Ok(Network(NetworkType::Main))
        } else if value == "test" {
            Ok(Network(NetworkType::Test))
        } else if value == "regtest" {
            Ok(Network(NetworkType::Regtest))
        } else {
            bail!("Invalid network identifier: {}", value)
        }
    }
}

impl From<Network> for CBOR {
    fn from(value: Network) -> Self {
        String::from(value).into()
    }
}

impl TryFrom<CBOR> for Network {
    type Error = anyhow::Error;

    fn try_from(cbor: CBOR) -> Result<Self, Self::Error> {
        cbor.try_into_text()?.try_into()
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
impl crate::RandomInstance for Network {
    fn random() -> Self {
        Network(match rand::random::<u8>() % 3 {
            0 => NetworkType::Main,
            1 => NetworkType::Test,
            _ => NetworkType::Regtest,
        })
    }
}

test_cbor_roundtrip!(Network);
test_envelope_roundtrip!(Network);
