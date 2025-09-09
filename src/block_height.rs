use bc_envelope::prelude::*;
use std::cmp::{Ord, Ordering};
use std::fmt;
use std::ops::{Add, Sub};

/// A block's position in the blockchain, represented as a distance from the genesis block.
///
/// `BlockHeight` represents the number of blocks between a specific block and the genesis
/// block (block zero). Each block increments the height by one, forming a total ordering
/// that defines the blockchain's canonical sequence.
///
/// # Zcash Concept Relation
/// In Zcash, like Bitcoin, block height is crucial for:
///
/// - Identifying when transactions were confirmed
/// - Determining when network upgrades activate
/// - Calculating confirmation counts
/// - Anchoring shielded transactions to specific blockchain states
///
/// Many Zcash protocol parameters are defined in terms of block heights, including
/// upgrade activation heights and consensus rule changes.
///
/// # Data Preservation
/// `BlockHeight` preserves the numeric height values from wallet data, which are essential
/// for chronological ordering of transactions and for determining whether specific network
/// features were active when a transaction was created.
///
/// # Implementation Details
/// Internally, block heights are stored as unsigned 32-bit integers, which can represent
/// blocks up to approximately 136 years into the future at Zcash's target rate of one
/// block every 75 seconds.
///
/// # Examples
/// ```
/// # use zewif::BlockHeight;
/// // The genesis block
/// let genesis = BlockHeight::from(0u32);
///
/// // Block #1,000,000
/// let millionth = BlockHeight::from(1_000_000u32);
///
/// // Calculate difference between blocks
/// let blocks_between = millionth - genesis;
/// assert_eq!(blocks_between, 1_000_000);
/// ```
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct BlockHeight(u32);

/// The height of the genesis block on a network.
pub const H0: BlockHeight = BlockHeight(0);

impl BlockHeight {
    /// Creates a new `BlockHeight` from a u32 value.
    ///
    /// This constructor is a `const fn`, which allows it to be used in constant expressions.
    ///
    /// # Examples
    /// ```
    /// # use zewif::BlockHeight;
    /// // Create a constant block height
    /// const CANOPY_ACTIVATION: BlockHeight = BlockHeight::from_u32(1_046_400);
    /// ```
    pub const fn from_u32(v: u32) -> BlockHeight {
        BlockHeight(v)
    }

    /// Subtracts the provided value from this height, returning `H0` if this would result in
    /// underflow of the wrapped `u32`.
    ///
    /// This method ensures that block height calculations never underflow below the genesis
    /// block (height 0), which would be an invalid state in a blockchain.
    ///
    /// # Examples
    /// ```
    /// # use zewif::BlockHeight;
    /// let height = BlockHeight::from(100u32);
    ///
    /// // Normal subtraction
    /// let earlier = height.saturating_sub(50);
    /// assert_eq!(u32::from(earlier), 50);
    ///
    /// // Saturating at genesis block (0) when underflow would occur
    /// let genesis = height.saturating_sub(200); // Would be -100, saturates to 0
    /// assert_eq!(u32::from(genesis), 0);
    /// ```
    pub fn saturating_sub(self, v: u32) -> BlockHeight {
        BlockHeight(self.0.saturating_sub(v))
    }
}

/// Displays the block height as a plain number
impl fmt::Display for BlockHeight {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

/// Implements a total ordering between block heights
impl Ord for BlockHeight {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

/// Implements a partial ordering between block heights
impl PartialOrd for BlockHeight {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Creates a BlockHeight from a u32 value
impl From<u32> for BlockHeight {
    fn from(value: u32) -> Self {
        BlockHeight(value)
    }
}

/// Extracts the u32 value from a BlockHeight
impl From<BlockHeight> for u32 {
    fn from(value: BlockHeight) -> u32 {
        value.0
    }
}

/// Creates a BlockHeight from a u64 value if it fits in a u32
impl TryFrom<u64> for BlockHeight {
    type Error = std::num::TryFromIntError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        u32::try_from(value).map(BlockHeight)
    }
}

/// Converts a BlockHeight to a u64 value
impl From<BlockHeight> for u64 {
    fn from(value: BlockHeight) -> u64 {
        value.0 as u64
    }
}

/// Creates a BlockHeight from a signed i32 if it's non-negative
impl TryFrom<i32> for BlockHeight {
    type Error = std::num::TryFromIntError;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        u32::try_from(value).map(BlockHeight)
    }
}

/// Creates a BlockHeight from a signed i64 if it fits in a u32
impl TryFrom<i64> for BlockHeight {
    type Error = std::num::TryFromIntError;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        u32::try_from(value).map(BlockHeight)
    }
}

/// Converts a BlockHeight to a signed i64 value
impl From<BlockHeight> for i64 {
    fn from(value: BlockHeight) -> i64 {
        value.0 as i64
    }
}

/// Adds a block count to a height, with saturation to prevent overflow
impl Add<u32> for BlockHeight {
    type Output = Self;

    fn add(self, other: u32) -> Self {
        BlockHeight(self.0.saturating_add(other))
    }
}

/// Subtracts a block count from a height, with saturation to prevent underflow
impl Sub<u32> for BlockHeight {
    type Output = Self;

    fn sub(self, other: u32) -> Self {
        BlockHeight(self.0.saturating_sub(other))
    }
}

/// Calculates the block count between two heights, with saturation
impl Sub<BlockHeight> for BlockHeight {
    type Output = u32;

    fn sub(self, other: BlockHeight) -> u32 {
        self.0.saturating_sub(other.0)
    }
}

impl From<BlockHeight> for CBOR {
    fn from(value: BlockHeight) -> Self {
        CBOR::from(value.0)
    }
}

impl From<&BlockHeight> for CBOR {
    fn from(value: &BlockHeight) -> Self {
        CBOR::from(value.0)
    }
}

impl TryFrom<CBOR> for BlockHeight {
    type Error = dcbor::Error;

    fn try_from(cbor: CBOR) -> dcbor::Result<Self> {
        Ok(BlockHeight::from(u32::try_from(cbor)?))
    }
}

impl From<BlockHeight> for Envelope {
    fn from(value: BlockHeight) -> Self {
        Envelope::new(CBOR::from(value))
    }
}

impl TryFrom<Envelope> for BlockHeight {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.extract_subject()
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_cbor_roundtrip, test_envelope_roundtrip};

    use super::BlockHeight;

    impl crate::RandomInstance for BlockHeight {
        fn random() -> Self {
            let mut rng = bc_rand::thread_rng();
            let value = rand::Rng::gen_range(&mut rng, 0..u32::MAX);
            Self(value)
        }
    }

    test_cbor_roundtrip!(BlockHeight);
    test_envelope_roundtrip!(BlockHeight);
}
