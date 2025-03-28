use anyhow::Result;

use crate::{parse, parser::prelude::*};

pub use zcash_protocol::consensus::BranchId;

impl Parse for BranchId {
    fn parse(p: &mut Parser) -> Result<Self> {
        let consensus_branch_id = parse!(p, u32, "consensus branch ID")?;
        BranchId::try_from(consensus_branch_id)
            .map_err(|_| anyhow::anyhow!("Unknown consensus branch ID: {}", consensus_branch_id))
    }
}
