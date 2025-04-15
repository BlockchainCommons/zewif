use anyhow::Context;
use bc_components::ARID;
use bc_envelope::prelude::*;
use std::collections::HashMap;

use crate::{Indexed, envelope_indexed_objects_for_predicate, test_envelope_roundtrip};

use super::{Transaction, TxId, ZewifWallet};

/// The top-level container for the Zcash Wallet Interchange Format (ZeWIF).
///
/// `ZewifTop` is the root structure of the ZeWIF hierarchy, serving as a container
/// for multiple wallets and a global transaction history. This structure represents
/// the entirety of the data that would be migrated between different Zcash wallet
/// implementations.
///
/// # Zcash Concept Relation
///
/// In the Zcash wallet ecosystem:
///
/// - **Interchange Container**: `ZewifTop` serves as the standardized format for
///   moving wallet data between different implementations
/// - **Multi-Wallet Support**: A single interchange file can contain multiple wallets,
///   each with its own accounts and configuration
/// - **Global Transaction History**: Transactions are stored at the top level and
///   referenced by accounts in wallets, avoiding duplication
/// - **Migration Target**: This structure is the complete package that can be
///   serialized/deserialized during wallet migration
///
/// # Data Preservation
///
/// During wallet migration, the ZeWIF top-level container preserves:
///
/// - **Complete Wallet Collection**: All wallets with their unique identities and configurations
/// - **Full Transaction Graph**: The complete transaction history across all wallets
/// - **Relationship Structure**: The connections between wallets, accounts, and transactions
/// - **Vendor-Specific Extensions**: Custom metadata through the attachments system
///
/// # Examples
/// ```no_run
/// # use zewif::{ZewifTop, ZewifWallet, Network, Transaction, TxId};
/// // Create the top-level container
/// let mut zewif = ZewifTop::new();
///
/// // Add a wallet
/// let wallet = ZewifWallet::new(Network::Main);
/// zewif.add_wallet(wallet);
///
/// // Add a transaction to the global history
/// let txid = TxId::from_bytes([0u8; 32]); // In practice, a real transaction ID
/// let tx = Transaction::new(txid);
/// zewif.add_transaction(txid, tx);
///
/// // Access transactions
/// let tx_count = zewif.transactions().len();
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct ZewifTop {
    id: ARID,
    wallets: Vec<ZewifWallet>,
    transactions: HashMap<TxId, Transaction>,
    attachments: Attachments,
}

bc_envelope::impl_attachable!(ZewifTop);

impl ZewifTop {
    pub fn new() -> Self {
        Self {
            id: ARID::new(),
            wallets: Vec::new(),
            transactions: HashMap::new(),
            attachments: Attachments::new(),
        }
    }

    pub fn wallets(&self) -> &Vec<ZewifWallet> {
        &self.wallets
    }

    pub fn wallets_len(&self) -> usize {
        self.wallets.len()
    }

    pub fn add_wallet(&mut self, mut wallet: ZewifWallet) {
        wallet.set_index(self.wallets_len());
        self.wallets.push(wallet);
    }

    pub fn transactions(&self) -> &HashMap<TxId, Transaction> {
        &self.transactions
    }

    pub fn add_transaction(&mut self, txid: TxId, transaction: Transaction) {
        self.transactions.insert(txid, transaction);
    }

    pub fn get_transaction(&self, txid: TxId) -> Option<&Transaction> {
        self.transactions.get(&txid)
    }

    pub fn set_transactions(&mut self, transactions: HashMap<TxId, Transaction>) {
        self.transactions = transactions;
    }
}

impl Default for ZewifTop {
    fn default() -> Self {
        Self::new()
    }
}

#[rustfmt::skip]
impl From<ZewifTop> for Envelope {
    fn from(value: ZewifTop) -> Self {
        let mut e = Envelope::new(value.id)
            .add_type("Zewif");
        e = value.wallets.iter().fold(e, |e, wallet| e.add_assertion("wallet", wallet.clone()));
        e = value.transactions.iter().fold(e, |e, (_, transaction)| e.add_assertion("transaction", transaction.clone()));
        value.attachments.add_to_envelope(e)
    }
}

#[rustfmt::skip]
impl TryFrom<Envelope> for ZewifTop {
    type Error = anyhow::Error;

    fn try_from(envelope: Envelope) -> Result<Self, Self::Error> {
        envelope.check_type_envelope("Zewif")?;
        let id = envelope.extract_subject()?;

        let wallets = envelope_indexed_objects_for_predicate(&envelope, "wallet")?;

        let transactions = envelope
            .try_objects_for_predicate::<Transaction>("transaction")?
            .into_iter().map(|tx| (tx.txid(), tx)).collect();

        let attachments = Attachments::try_from_envelope(&envelope).context("attachments")?;

        Ok(Self {
            id,
            wallets,
            transactions,
            attachments,
        })
    }
}

#[cfg(test)]
#[rustfmt::skip]
impl crate::RandomInstance for ZewifTop {
    fn random() -> Self {
        use crate::SetIndexes;

        Self {
            id: ARID::new(),
            wallets: Vec::random().set_indexes(),
            transactions: Vec::<Transaction>::random().iter().map(|tx| (tx.txid(), tx.clone())).collect(),
            attachments: Attachments::random(),
        }
    }
}

test_envelope_roundtrip!(ZewifTop);
