use bc_envelope::{Envelope, prelude::CBOR};
use dcbor::prelude::*;

use crate::BlockHash;

/// The unique identifier of a transaction on the blockchain in terms of the hash of the block that
/// includes it and the index of the transaction within the block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxBlockPosition {
    /// The hash of the block containing the transaction.
    block_hash: BlockHash,
    /// The 0-based index of the transaction within the block.
    index: u32,
}

impl TxBlockPosition {
    pub fn new(block_hash: BlockHash, index: u32) -> Self {
        Self { block_hash, index }
    }

    pub fn block_hash(&self) -> &BlockHash {
        &self.block_hash
    }

    pub fn index(&self) -> u32 {
        self.index
    }
}

impl From<TxBlockPosition> for CBOR {
    fn from(value: TxBlockPosition) -> Self {
        let mut map = Map::new();
        map.insert("block_hash", value.block_hash);
        map.insert("index", value.index);
        map.into()
    }
}

impl TryFrom<CBOR> for TxBlockPosition {
    type Error = dcbor::Error;

    fn try_from(value: CBOR) -> dcbor::Result<Self> {
        if let CBORCase::Map(map) = value.into_case() {
            let block_hash: BlockHash = map.extract("block_hash")?;
            let index: u32 = map.extract("index")?;
            Ok(TxBlockPosition { block_hash, index })
        } else {
            Err("Expected a CBOR map".into())
        }
    }
}

impl From<TxBlockPosition> for Envelope {
    fn from(value: TxBlockPosition) -> Self {
        Envelope::new(CBOR::from(value)).add_type("TxBlockPosition")
    }
}

impl TryFrom<Envelope> for TxBlockPosition {
    type Error = bc_envelope::Error;

    fn try_from(value: Envelope) -> bc_envelope::Result<Self> {
        value.check_type_envelope("TxBlockPosition")?;
        value.extract_subject()
    }
}

#[cfg(test)]
mod envelope_tests {
    use crate::{BlockHash, test_envelope_roundtrip};

    use super::TxBlockPosition;

    impl crate::RandomInstance for TxBlockPosition {
        fn random() -> Self {
            Self {
                block_hash: BlockHash::random(),
                index: u32::random(),
            }
        }
    }

    test_envelope_roundtrip!(TxBlockPosition);
}
