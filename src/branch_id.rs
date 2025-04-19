use std::fmt::Display;

use anyhow::Result;
use bc_envelope::prelude::*;

use crate::{parse, parser::prelude::*, test_cbor_roundtrip};

/// Identifies the consensus rules in effect for a particular block or transaction.
///
/// `BranchId` represents the different network upgrades in Zcash's history, each of which
/// introduced new consensus rules and protocol features. The branch ID is used for
/// transaction signature validation and to determine which consensus rules apply
/// at a given block height.
///
/// # Zcash Concept Relation
/// Zcash evolves through planned network upgrades that activate at specific block heights.
/// Each upgrade has a unique ID (branch ID) used to:
///
/// - **Replay protection**: Prevents transactions from one network upgrade being valid on another
/// - **Versioning**: Indicates which transaction format and validation rules apply
/// - **Activation**: Determines which features are available based on block height
///
/// Unlike most blockchains that use simple version numbers, Zcash uses unique 32-bit values
/// for each upgrade to ensure strong transaction replay protection between different
/// network upgrade eras.
///
/// # Data Preservation
/// During wallet migration, preserving branch IDs is critical for:
/// - Transaction validation, especially for partially signed transactions
/// - Correctly applying activation rules to wallet data
/// - Understanding which privacy and consensus features were available when transactions occurred
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BranchId(zcash_protocol::consensus::BranchId);

/// Converts a 32-bit numeric branch ID into the corresponding enum variant
impl TryFrom<u32> for BranchId {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        zcash_protocol::consensus::BranchId::try_from(value).map(BranchId)
    }
}

/// Converts a BranchId enum variant to its raw 32-bit numeric value
impl From<BranchId> for u32 {
    fn from(consensus_branch_id: BranchId) -> u32 {
        u32::from(consensus_branch_id.0)
    }
}

/// Parses a BranchId from a binary data stream
impl Parse for BranchId {
    fn parse(p: &mut Parser) -> Result<Self> {
        let consensus_branch_id = parse!(p, u32, "BranchId")?;
        BranchId::try_from(consensus_branch_id)
            .map_err(|_| anyhow::anyhow!("Unknown BranchId: {}", consensus_branch_id))
    }
}

impl Display for BranchId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            zcash_protocol::consensus::BranchId::Sprout => write!(f, "Sprout"),
            zcash_protocol::consensus::BranchId::Overwinter => write!(f, "Overwinter"),
            zcash_protocol::consensus::BranchId::Sapling => write!(f, "Sapling"),
            zcash_protocol::consensus::BranchId::Blossom => write!(f, "Blossom"),
            zcash_protocol::consensus::BranchId::Heartwood => write!(f, "Heartwood"),
            zcash_protocol::consensus::BranchId::Canopy => write!(f, "Canopy"),
            zcash_protocol::consensus::BranchId::Nu5 => write!(f, "Nu5"),
            zcash_protocol::consensus::BranchId::Nu6 => write!(f, "Nu6"),
        }
    }
}

impl From<BranchId> for CBOR {
    fn from(value: BranchId) -> Self {
        u32::from(value).into()
    }
}

impl TryFrom<CBOR> for BranchId {
    type Error = anyhow::Error;

    fn try_from(cbor: CBOR) -> Result<Self> {
        BranchId::try_from(u32::try_from(cbor)?).map_err(|e| anyhow::anyhow!(e))
    }
}

#[cfg(test)]
impl crate::RandomInstance for BranchId {
    fn random() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let random_value: u32 = rng.gen_range(0..=8);
        BranchId(match random_value {
            0 => zcash_protocol::consensus::BranchId::Sprout,
            1 => zcash_protocol::consensus::BranchId::Overwinter,
            2 => zcash_protocol::consensus::BranchId::Sapling,
            3 => zcash_protocol::consensus::BranchId::Blossom,
            4 => zcash_protocol::consensus::BranchId::Heartwood,
            5 => zcash_protocol::consensus::BranchId::Canopy,
            6 => zcash_protocol::consensus::BranchId::Nu5,
            7 => zcash_protocol::consensus::BranchId::Nu6,
            _ => unreachable!(),
        })
    }
}

test_cbor_roundtrip!(BranchId);
