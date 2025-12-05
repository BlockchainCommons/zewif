use super::{BlockHeight, Data, TxId};
use crate::TxBlockPosition;
use bc_envelope::prelude::*;

/// A Zcash transaction that can combine transparent and multiple shielded protocol components.
///
/// `Transaction` represents a complete Zcash transaction, which can include components from
/// transparent Bitcoin-style inputs/outputs as well as from any of the three Zcash shielded
/// protocols: Sprout, Sapling, and Orchard. The transaction structure preserves both blockchain
/// data (block height, timestamp) and the detailed components needed to represent the
/// transaction in a wallet format.
///
/// # Zcash Concept Relation
///
/// In Zcash:
///
/// - **Unified Transaction Format**: Transactions can seamlessly combine transparent and
///   different shielded protocols in a single operation
/// - **Multi-Protocol Support**: Zcash has evolved through multiple shielded protocols,
///   and transactions can include components from any of them:
///   - Transparent (Bitcoin-style public inputs/outputs)
///   - Sprout (original shielded protocol using JoinSplits)
///   - Sapling (improved shielded protocol with separate spends/outputs)
///   - Orchard (latest shielded protocol using unified actions)
/// - **Transaction Lifecycle**: Transactions go through stages (Pending, Confirmed, Failed, Abandoned)
///   that represent their status on the blockchain
///
/// # Data Preservation
///
/// During wallet migration, the following transaction data must be preserved:
///
/// - **Identity**: The unique transaction ID (txid)
/// - **Blockchain Context**: Block height, timestamp, block hash when available
/// - **Status Information**: Whether the transaction is pending, confirmed, failed, or abandoned
/// - **Raw Transaction**: Optional full binary transaction data
/// - **Protocol-Specific Components**:
///   - Transparent inputs and outputs
///   - Sapling spends and outputs
///   - Orchard actions
///   - Sprout JoinSplits
///
/// # Examples
/// ```no_run
/// # use zewif::{Transaction, TxId, BlockHeight};
/// // Create a new transaction with a transaction ID (in practice, a real ID)
/// let txid = TxId::from_bytes([0u8; 32]);
/// let mut tx = Transaction::new(txid);
///
/// // Set transaction metadata
/// tx.set_mined_height(BlockHeight::from(1000000));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Transaction {
    /// The transaction id.
    txid: TxId,
    /// The raw transaction data, if known.
    raw: Option<Data>,
    /// The height for which the transaction was constructed, which implies
    /// the consensus branch for which the transaction was intended, if known.
    target_height: Option<BlockHeight>,
    /// The height at which the transaction was mined, if known.
    /// It is possible that if a rollback occurred just after the zeWIF
    /// export, the transaction could have been unmined, and possibly
    /// remined at a different height.
    mined_height: Option<BlockHeight>,
    /// The hash of the block containing the transaction and the index of the transaction within
    /// the block, if known.
    block_position: Option<TxBlockPosition>,
    /// Additional arbitrary metadata related to the transaction.
    attachments: Attachments,
}

bc_envelope::impl_attachable!(Transaction);

impl Transaction {
    pub fn new(txid: TxId) -> Self {
        Self {
            txid,
            raw: None,
            target_height: None,
            mined_height: None,
            block_position: None,
            attachments: Attachments::new(),
        }
    }

    pub fn txid(&self) -> TxId {
        self.txid
    }

    pub fn set_txid(&mut self, txid: TxId) {
        self.txid = txid;
    }

    pub fn raw(&self) -> Option<&Data> {
        self.raw.as_ref()
    }

    pub fn set_raw(&mut self, raw: Data) {
        self.raw = Some(raw);
    }

    pub fn target_height(&self) -> Option<&BlockHeight> {
        self.target_height.as_ref()
    }

    pub fn set_target_height(&mut self, height: BlockHeight) {
        self.target_height = Some(height);
    }

    pub fn mined_height(&self) -> Option<&BlockHeight> {
        self.mined_height.as_ref()
    }

    pub fn set_mined_height(&mut self, height: BlockHeight) {
        self.mined_height = Some(height);
    }

    pub fn block_position(&self) -> Option<&TxBlockPosition> {
        self.block_position.as_ref()
    }

    pub fn set_block_position(&mut self, block_position: Option<TxBlockPosition>) {
        self.block_position = block_position;
    }
}

#[rustfmt::skip]
impl From<Transaction> for Envelope {
    fn from(value: Transaction) -> Self {
        let e = Envelope::new(value.txid)
            .add_type("Transaction")
            .add_optional_assertion("raw", value.raw)
            .add_optional_assertion("target_height", value.target_height)
            .add_optional_assertion("mined_height", value.mined_height)
            .add_optional_assertion("block_position", value.block_position);
        value.attachments.add_to_envelope(e)
    }
}

impl TryFrom<Envelope> for Transaction {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("Transaction")?;
        let txid = envelope.extract_subject()?;
        let raw = envelope.try_optional_object_for_predicate("raw")?;
        let target_height = envelope.try_optional_object_for_predicate("target_height")?;
        let mined_height = envelope.try_optional_object_for_predicate("mined_height")?;
        let block_position = envelope.try_optional_object_for_predicate("block_position")?;
        let attachments = Attachments::try_from_envelope(&envelope)
            .map_err(|e| bc_envelope::Error::General(format!("attachments: {}", e)))?;

        Ok(Self {
            txid,
            raw,
            target_height,
            mined_height,
            block_position,
            attachments,
        })
    }
}

#[cfg(test)]
mod tests {
    use bc_envelope::Attachments;

    use super::Transaction;
    use crate::{BlockHeight, Data, TxBlockPosition, TxId, test_envelope_roundtrip};

    impl crate::RandomInstance for Transaction {
        fn random() -> Self {
            Self {
                txid: TxId::random(),
                raw: Data::opt_random(),
                target_height: BlockHeight::opt_random(),
                mined_height: BlockHeight::opt_random(),
                block_position: TxBlockPosition::opt_random(),
                attachments: Attachments::random(),
            }
        }
    }

    test_envelope_roundtrip!(Transaction);
}
