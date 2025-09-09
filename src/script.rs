use super::Data;
use bc_envelope::prelude::*;
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
/// # use zewif::{Script, Data};
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

impl From<Script> for CBOR {
    fn from(value: Script) -> Self {
        CBOR::to_byte_string(value.0)
    }
}

impl From<&Script> for CBOR {
    fn from(value: &Script) -> Self {
        CBOR::to_byte_string(value.0.clone())
    }
}

impl TryFrom<CBOR> for Script {
    type Error = dcbor::Error;

    fn try_from(cbor: CBOR) -> dcbor::Result<Self> {
        let bytes = cbor.try_into_byte_string()?;
        if bytes.len() > 0xffff {
            return Err("Script length exceeds maximum size of 65535 bytes".into());
        }
        Ok(Script(Data::from_vec(bytes)))
    }
}

impl From<Script> for Envelope {
    fn from(value: Script) -> Self {
        Envelope::new(CBOR::from(value))
    }
}

impl TryFrom<Envelope> for Script {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.extract_subject()
    }
}

#[cfg(test)]
mod tests {
    use crate::{Data, test_cbor_roundtrip, test_envelope_roundtrip};

    use super::Script;

    impl crate::RandomInstance for Script {
        fn random_with_size(size: usize) -> Self {
            Self(Data::random_with_size(size))
        }

        fn random() -> Self {
            Self(Data::random())
        }
    }

    test_cbor_roundtrip!(Script);
    test_envelope_roundtrip!(Script);
}
