use anyhow::Result;

use super::Data;
use crate::{parse, parser::prelude::*};
use std::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};

/// A Bitcoin-style script for spending or encumbering coins in transparent transactions.
///
/// `Script` represents a serialized Bitcoin script, which is a sequence of operations used to 
/// specify conditions for spending Zcash coins in transparent transactions. In ZeWIF,
/// scripts appear in two contexts:
///
/// - `script_pubkey` in outputs: Defines spending conditions (e.g., P2PKH, P2SH)
/// - `script_sig` in inputs: Contains signatures and data to satisfy spending conditions
/// 
/// Internally, `Script` is a wrapper around [`Data`](crate::Data), providing a 
/// type-safe representation for script handling.
///
/// # Zcash Concept Relation
/// Zcash inherits the Bitcoin script system for its transparent UTXO model. Unlike
/// shielded transactions, which use zero-knowledge proofs, transparent transactions
/// use explicit scripts to validate spending conditions.
///
/// Common script patterns in Zcash transparent addresses include:
/// - Pay to Public Key Hash (P2PKH): Sends to a standard transparent address
/// - Pay to Script Hash (P2SH): Sends to a script hash, enabling more complex conditions
///
/// # Data Preservation
/// The `Script` type preserves the exact binary representation of transaction scripts
/// from wallet data, ensuring cryptographic integrity during wallet migrations.
///
/// # Examples
/// ```
/// use zewif::{Script, Data};
///
/// // Create a script from binary data (this would typically be from a transaction)
/// let script_bytes = vec![0x76, 0xa9, 0x14, /* more script bytes */];
/// let script = Script::from(Data::from_vec(script_bytes.clone()));
///
/// // Check script properties
/// assert_eq!(script.len(), script_bytes.len());
/// assert!(!script.is_empty());
/// ```
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Script(Data);

impl Script {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Parses a Script from a binary data stream
impl Parse for Script {
    fn parse(p: &mut Parser) -> Result<Self> {
        Ok(Self(parse!(p, "Script")?))
    }
}

/// Debug formatting that includes script length and hex representation
impl std::fmt::Debug for Script {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Script<{}>({})", self.0.len(), hex::encode(self))
    }
}

/// Allows treating a Script as a byte slice
impl AsRef<[u8]> for Script {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

/// Converts a Script to a Data value, allowing manipulation as variable-length bytes
impl From<Script> for Data {
    fn from(script: Script) -> Self {
        script.0
    }
}

/// Creates a Script from Data, allowing conversion from variable-length bytes
impl From<Data> for Script {
    fn from(data: Data) -> Self {
        Script(data)
    }
}

/// Allows accessing individual bytes in the script by index
impl Index<usize> for Script {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

/// Allows modifying individual bytes in the script by index
impl IndexMut<usize> for Script {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<Range<usize>> for Script {
    type Output = [u8];

    fn index(&self, index: Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<Range<usize>> for Script {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<RangeTo<usize>> for Script {
    type Output = [u8];

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<RangeTo<usize>> for Script {
    fn index_mut(&mut self, index: RangeTo<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<RangeFrom<usize>> for Script {
    type Output = [u8];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<RangeFrom<usize>> for Script {
    fn index_mut(&mut self, index: RangeFrom<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<RangeFull> for Script {
    type Output = [u8];

    fn index(&self, index: RangeFull) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<RangeFull> for Script {
    fn index_mut(&mut self, index: RangeFull) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<RangeInclusive<usize>> for Script {
    type Output = [u8];

    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<RangeInclusive<usize>> for Script {
    fn index_mut(&mut self, index: RangeInclusive<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<RangeToInclusive<usize>> for Script {
    type Output = [u8];

    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<RangeToInclusive<usize>> for Script {
    fn index_mut(&mut self, index: RangeToInclusive<usize>) -> &mut Self::Output {
        &mut self.0[index]
    }
}
