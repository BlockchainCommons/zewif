use crate::error::Error;
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

impl From<Network> for String {
    fn from(value: Network) -> String {
        match value {
            Network::Main => "main".to_string(),
            Network::Test => "test".to_string(),
            Network::Regtest => "regtest".to_string(),
        }
    }
}

impl TryFrom<String> for Network {
    type Error = Error;

    fn try_from(value: String) -> crate::error::Result<Self> {
        if value == "main" {
            Ok(Network::Main)
        } else if value == "test" {
            Ok(Network::Test)
        } else if value == "regtest" {
            Ok(Network::Regtest)
        } else {
            Err(Error::InvalidNetwork(value))
        }
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
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        let network_str: String = envelope.extract_subject()?;
        Network::try_from(network_str).map_err(|e| e.into())
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
