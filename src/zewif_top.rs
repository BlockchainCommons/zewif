use std::collections::HashMap;

use bc_components::ARID;

use crate::impl_attachable;
use super::{Attachments, Transaction, TxId, ZewifWallet};

/// Represents a Zewif wallet database, the top level of the interchange format
/// hierarchy, which can contain multiple wallets and a global transaction
/// history.
#[derive(Debug, Clone)]
pub struct ZewifTop {
    wallets: HashMap<ARID, ZewifWallet>,
    transactions: HashMap<TxId, Transaction>,
    attachments: Attachments,
}

impl_attachable!(ZewifTop);

impl ZewifTop {
    pub fn new() -> Self {
        Self {
            wallets: HashMap::new(),
            transactions: HashMap::new(),
            attachments: Attachments::new(),
        }
    }

    pub fn wallets(&self) -> &HashMap<ARID, ZewifWallet> {
        &self.wallets
    }

    pub fn add_wallet(&mut self, wallet: ZewifWallet) {
        self.wallets.insert(wallet.id().clone(), wallet);
    }

    pub fn transactions(&self) -> &HashMap<TxId, Transaction> {
        &self.transactions
    }

    pub fn transactions_mut(&mut self) -> &mut HashMap<TxId, Transaction> {
        &mut self.transactions
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
