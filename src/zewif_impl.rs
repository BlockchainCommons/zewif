use bc_components::ARID;
use bc_envelope::prelude::*;
use std::collections::HashMap;

use crate::{BlockHeight, Indexed, envelope_indexed_objects_for_predicate};

use super::{Transaction, TxId, ZewifWallet};

/// The top-level container for the Zcash Wallet Interchange Format (ZeWIF).
///
/// `Zewif` is the root structure of the ZeWIF hierarchy, serving as a container
/// for multiple wallets and a global transaction history. This structure represents
/// the entirety of the data that would be migrated between different Zcash wallet
/// implementations.
///
/// # Zcash Concept Relation
///
/// In the Zcash wallet ecosystem:
///
/// - **Interchange Container**: `Zewif` serves as the standardized format for
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
/// # use zewif::{Zewif, ZewifWallet, Network, Transaction, TxId, BlockHeight};
/// // Create the top-level container
/// let mut zewif = Zewif::new(BlockHeight::from_u32(2000000));
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
pub struct Zewif {
    id: ARID,
    wallets: Vec<ZewifWallet>,
    transactions: HashMap<TxId, Transaction>,
    export_height: BlockHeight,
    attachments: Attachments,
}

bc_envelope::impl_attachable!(Zewif);

impl Zewif {
    pub fn new(export_height: BlockHeight) -> Self {
        Self {
            id: ARID::new(),
            wallets: Vec::new(),
            transactions: HashMap::new(),
            export_height,
            attachments: Attachments::new(),
        }
    }

    pub fn id(&self) -> ARID {
        self.id
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

    pub fn export_height(&self) -> BlockHeight {
        self.export_height
    }
}

#[rustfmt::skip]
impl From<Zewif> for Envelope {
    fn from(value: Zewif) -> Self {
        let mut e = Envelope::new(value.id)
            .add_type("Zewif");
        e = value.wallets.iter().fold(e, |e, wallet| e.add_assertion("wallet", wallet.clone()));
        e = value.transactions.iter().fold(e, |e, (_, transaction)| e.add_assertion("transaction", transaction.clone()));
        e = e.add_assertion("export_height", value.export_height);
        value.attachments.add_to_envelope(e)
    }
}

#[rustfmt::skip]
impl TryFrom<Envelope> for Zewif {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type_envelope("Zewif")?;
        let id = envelope.extract_subject()?;

        let wallets = envelope_indexed_objects_for_predicate(&envelope, "wallet")
            .map_err(|e| bc_envelope::Error::General(format!("wallets: {}", e)))?;

        let transactions = envelope
            .try_objects_for_predicate::<Transaction>("transaction")?
            .into_iter().map(|tx| (tx.txid(), tx)).collect();

        let export_height = envelope.extract_object_for_predicate("export_height")?;
        let attachments = Attachments::try_from_envelope(&envelope)
            .map_err(|e| bc_envelope::Error::General(format!("attachments: {}", e)))?;

        Ok(Self {
            id,
            wallets,
            transactions,
            export_height,
            attachments,
        })
    }
}

#[cfg(test)]
mod tests {
    use bc_components::ARID;
    use bc_envelope::Attachments;

    use crate::{BlockHeight, Transaction, test_envelope_roundtrip};

    use super::Zewif;

    impl crate::RandomInstance for Zewif {
        fn random() -> Self {
            use crate::SetIndexes;

            Self {
                id: ARID::new(),
                wallets: Vec::random().set_indexes(),
                transactions: Vec::<Transaction>::random()
                    .iter()
                    .map(|tx| (tx.txid(), tx.clone()))
                    .collect(),
                export_height: BlockHeight::random(),
                attachments: Attachments::random(),
            }
        }
    }

    test_envelope_roundtrip!(Zewif);
}
