use std::collections::HashMap;

use bc_components::ARID;

use crate::{impl_attachable, NoQuotesDebugOption};
use super::Network;

use super::{Account, Attachments, Identifiable, SeedMaterial};

/// Represents an entire wallet, including multiple accounts, a wallet-specific
/// subset of the global transaction history, and optionally a form of seed
/// material for generating cryptographic keys.
///
/// This is *not* the top level of the interchange format hierarchy. That is
/// the `ZewifTop` type.
#[derive(Clone)]
pub struct ZewifWallet {
    id: ARID,
    network: Network,
    seed_material: Option<SeedMaterial>,
    accounts: HashMap<ARID, Account>,
    attachments: Attachments,
}

impl std::fmt::Debug for ZewifWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ZewifWallet")
            .field("id", &self.id)
            .field("network", &self.network)
            .field("seed_material", &NoQuotesDebugOption(&self.seed_material))
            .field("accounts", &self.accounts)
            .field("attachments", &self.attachments)
            .finish()
    }
}

impl Identifiable for ZewifWallet {
    fn id(&self) -> &ARID {
        &self.id
    }
}

impl_attachable!(ZewifWallet);

impl ZewifWallet {
    pub fn new(network: Network) -> Self {
        Self {
            id: ARID::new(),
            network,
            seed_material: None,
            accounts: HashMap::new(),
            attachments: Attachments::new(),
        }
    }

    pub fn id(&self) -> &ARID {
        &self.id
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

    pub fn accounts(&self) -> &HashMap<ARID, Account> {
        &self.accounts
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.insert(account.id().clone(), account);
    }
}
