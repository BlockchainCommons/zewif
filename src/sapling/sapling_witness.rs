use bc_envelope::prelude::*;

use crate::{IncrementalWitness, blob, blob_envelope};

/// The depth of the Zcash Sapling note commitment tree.
const SAPLING_COMMITMENT_TREE_DEPTH: usize = 32;

blob!(
    MerkleHashSapling,
    32,
    "A node in the Sapling note commitment tree."
);
impl Copy for MerkleHashSapling {}

blob_envelope!(MerkleHashSapling);

/// A cryptographic witness proving that a Sapling note commitment exists in the note commitment tree.
///
/// `SaplingWitness` is a specialized form of incremental Merkle tree witness for the
/// Sapling protocol. It proves that a specific note commitment is included in the
/// global Sapling note commitment tree, which is necessary when spending a note.
///
/// # Zcash Concept Relation
/// In Zcash's Sapling protocol:
///
/// - **Note Commitment Tree**: A Merkle tree containing all Sapling note commitments
/// - **Merkle Path**: The path from a leaf (note commitment) to the root of the tree
/// - **Witness**: The authentication path proving a leaf exists in the tree
/// - **Anchors**: Root hashes of the note commitment tree at specific blockchain heights
///
/// When spending a Sapling note, a zero-knowledge proof must demonstrate that the
/// note's commitment exists in the tree at a specific anchor (root hash), without
/// revealing which specific commitment is being spent. The witness provides the
/// necessary path information to create this proof.
///
/// # Data Preservation
/// During wallet migration, complete witness data must be preserved for all unspent
/// notes. This includes:
///
/// - The authentication path (sequence of hashes forming the Merkle path)
/// - The position of the note commitment in the tree
/// - The tree depth used (32 for Sapling)
///
/// Without this witness data, unspent notes cannot be spent as it would be impossible
/// to prove their inclusion in the note commitment tree.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SaplingWitness(
    IncrementalWitness<SAPLING_COMMITMENT_TREE_DEPTH, MerkleHashSapling>,
);

impl From<SaplingWitness> for Envelope {
    fn from(value: SaplingWitness) -> Self {
        Envelope::new(*value.0.note_commitment())
            .add_type("SaplingWitness")
            .add_assertion("note_position", value.0.note_position())
            .add_assertion("merkle_path", value.0.merkle_path().to_vec())
            .add_assertion("anchor", *value.0.anchor())
            .add_assertion("anchor_tree_size", value.0.anchor_tree_size())
            .add_assertion(
                "anchor_frontier",
                value.0.anchor_frontier().to_vec(),
            )
    }
}

impl TryFrom<Envelope> for SaplingWitness {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type_envelope("SaplingWitness")?;
        let note_commitment =
            envelope.extract_subject::<MerkleHashSapling>()?;
        let note_position =
            envelope.extract_object_for_predicate("note_position")?;
        let merkle_path =
            envelope.extract_object_for_predicate("merkle_path")?;
        let anchor = envelope.extract_object_for_predicate("anchor")?;
        let anchor_tree_size =
            envelope.extract_object_for_predicate("anchor_tree_size")?;
        let anchor_frontier =
            envelope.extract_object_for_predicate("anchor_frontier")?;
        Ok(Self(IncrementalWitness::from_parts(
            note_commitment,
            note_position,
            merkle_path,
            anchor,
            anchor_tree_size,
            anchor_frontier,
        )))
    }
}

#[cfg(test)]
mod tests {
    use crate::{IncrementalWitness, RandomInstance, test_envelope_roundtrip};

    use super::SaplingWitness;

    impl RandomInstance for SaplingWitness {
        fn random() -> Self {
            Self(IncrementalWitness::random())
        }
    }

    test_envelope_roundtrip!(SaplingWitness);
}
