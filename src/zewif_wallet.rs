use super::Network;
use super::{Account, SeedMaterial};
use crate::{
    Indexed, NoQuotesDebugOption, envelope_indexed_objects_for_predicate,
};
use bc_envelope::prelude::*;

/// A complete Zcash wallet with multiple accounts and cryptographic key material.
///
/// `ZewifWallet` represents an entire wallet consisting of multiple accounts, all operating
/// on the same Zcash network. It can optionally include seed material for generating keys.
/// This structure is the primary container for user wallet data but is not the top level
/// of the interchange format hierarchy (that's `Zewif`).
///
/// # Zcash Concept Relation
///
/// In Zcash wallet architecture:
///
/// - **Network Context**: Wallets operate within a specific Zcash network environment
///   (mainnet, testnet, regtest)
/// - **Multi-Account Organization**: A single wallet can contain multiple accounts
///   for different purposes or users
/// - **HD Wallet Structure**: When a seed is present, the wallet follows hierarchical
///   deterministic (HD) principles for key derivation
///
/// # Data Preservation
///
/// During wallet migration, the following wallet data must be preserved:
///
/// - **Network**: The Zcash network context (mainnet, testnet, regtest)
/// - **Seed Material**: When available, the cryptographic material used for key generation
/// - **Accounts**: All accounts contained within the wallet, with their full structure
/// - **Vendor-Specific Information**: Custom metadata stored in attachments
///
/// # Examples
/// ```no_run
/// # use zewif::{ZewifWallet, Network, Account, SeedMaterial};
/// // Create a new wallet for the Zcash mainnet
/// let mut wallet = ZewifWallet::new(Network::Main);
///
/// // Add a new account to the wallet
/// let account = Account::new();
/// wallet.add_account(account);
///
/// // Access the wallet's network
/// assert_eq!(wallet.network(), Network::Main);
///
/// // If seed material were available, you could add it:
/// // wallet.set_seed_material(seed_material);
/// ```
#[derive(Clone, PartialEq)]
pub struct ZewifWallet {
    index: usize,
    network: Network,
    seed_material: Option<SeedMaterial>,
    accounts: Vec<Account>,
    attachments: Attachments,
}

impl Indexed for ZewifWallet {
    fn index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

impl std::fmt::Debug for ZewifWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZewifWallet")
            .field("index", &self.index)
            .field("network", &self.network)
            .field("seed_material", &NoQuotesDebugOption(&self.seed_material))
            .field("accounts", &self.accounts)
            .field("attachments", &self.attachments)
            .finish()
    }
}

bc_envelope::impl_attachable!(ZewifWallet);

impl ZewifWallet {
    pub fn new(network: Network) -> Self {
        Self {
            index: 0,
            network,
            seed_material: None,
            accounts: Vec::new(),
            attachments: Attachments::new(),
        }
    }

    pub fn network(&self) -> Network {
        self.network
    }

    pub fn seed_material(&self) -> Option<&SeedMaterial> {
        self.seed_material.as_ref()
    }

    pub fn set_seed_material(&mut self, seed_material: SeedMaterial) {
        self.seed_material = Some(seed_material);
    }

    pub fn accounts(&self) -> &Vec<Account> {
        &self.accounts
    }

    pub fn add_account(&mut self, mut account: Account) {
        account.set_index(self.accounts.len());
        self.accounts.push(account);
    }
}

#[rustfmt::skip]
impl From<ZewifWallet> for Envelope {
    fn from(value: ZewifWallet) -> Self {
        let mut e = Envelope::new(value.index)
            .add_type("ZewifWallet")
            .add_assertion("network", value.network)
            .add_optional_assertion("seed_material", value.seed_material);

        e = value.accounts.iter().fold(e, |e, account| e.add_assertion("account", account.clone()));

        value.attachments.add_to_envelope(e)
    }
}

#[rustfmt::skip]
impl TryFrom<Envelope> for ZewifWallet {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("ZewifWallet")?;
        let index = envelope.extract_subject()?;
        let network = envelope.extract_object_for_predicate("network")?;
        let seed_material = envelope.try_optional_object_for_predicate("seed_material")?;

        let accounts = envelope_indexed_objects_for_predicate(&envelope, "account")
            .map_err(|e| bc_envelope::Error::General(format!("accounts: {}", e)))?;

        let attachments = Attachments::try_from_envelope(&envelope)
            .map_err(|e| bc_envelope::Error::General(format!("attachments: {}", e)))?;

        Ok(Self {
            index,
            network,
            seed_material,
            accounts,
            attachments,
        })
    }
}

#[cfg(test)]
mod tests {
    use bc_envelope::Attachments;

    use crate::{Network, SeedMaterial, test_envelope_roundtrip};

    use super::ZewifWallet;

    impl crate::RandomInstance for ZewifWallet {
        fn random() -> Self {
            use crate::SetIndexes;

            Self {
                index: 0,
                network: Network::random(),
                seed_material: SeedMaterial::opt_random(),
                accounts: Vec::random().set_indexes(),
                attachments: Attachments::random(),
            }
        }
    }

    test_envelope_roundtrip!(ZewifWallet);
}
