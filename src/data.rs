use std::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo,
    RangeToInclusive,
};

use crate::error::Result;
use bc_envelope::prelude::*;

/// A variable-size byte array wrapper for safely handling binary data of arbitrary length.
///
/// `Data` provides a flexible container for binary data that doesn't have a fixed size,
/// such as:
///
/// - Encrypted memo fields
/// - Variable-length signatures
/// - Script data
/// - Other protocol elements that can vary in size
///
/// Unlike [`Blob<N>`](crate::Blob), which is for fixed-size data, `Data` can hold
/// any amount of bytes and can grow or shrink as needed.
///
/// # Zcash Concept Relation
/// Zcash uses variable-length data structures in many parts of the protocol:
/// - Encrypted memos in shielded transactions
/// - Script data in transparent transactions
/// - Signatures and other cryptographic proofs
///
/// # Data Preservation
/// `Data` preserves variable-length binary blobs exactly as they appear in wallet files,
/// ensuring that encrypted data, scripts, and other variable-length content maintain
/// their exact representation during the migration process.
///
/// # Examples
/// ```
/// # use zewif::Data;
/// // Create from raw bytes
/// let bytes = vec![1, 2, 3, 4, 5];
/// let data = Data::from_vec(bytes.clone());
/// assert_eq!(data.len(), 5);
///
/// // Access via indexing
/// assert_eq!(data[0], 1);
/// assert_eq!(&data[1..3], &[2, 3]);
/// ```
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Data(Vec<u8>);

impl Data {
    /// Creates a new empty `Data` instance.
    ///
    /// # Examples
    /// ```
    /// # use zewif::Data;
    /// let data = Data::new();
    /// assert!(data.is_empty());
    /// ```
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates a `Data` instance from anything that can be referenced as a byte slice.
    ///
    /// This is a convenience method that allows creating a `Data` from various
    /// byte-containing types.
    ///
    /// # Examples
    /// ```
    /// # use zewif::Data;
    /// // From a slice
    /// let data1 = Data::from_bytes(&[1, 2, 3]);
    ///
    /// // From a Vec
    /// let data2 = Data::from_bytes(vec![1, 2, 3]);
    ///
    /// // From another Data instance
    /// let data3 = Data::from_bytes(&data1);
    ///
    /// assert_eq!(data1, data2);
    /// assert_eq!(data2, data3);
    /// ```
    pub fn from_bytes(data: impl AsRef<[u8]>) -> Self {
        Self(data.as_ref().to_vec())
    }

    /// Returns the number of bytes in the data.
    ///
    /// # Examples
    /// ```
    /// # use zewif::Data;
    /// let data = Data::from_bytes(&[1, 2, 3]);
    /// assert_eq!(data.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the data contains no bytes.
    ///
    /// # Examples
    /// ```
    /// # use zewif::Data;
    /// let empty = Data::new();
    /// assert!(empty.is_empty());
    ///
    /// let non_empty = Data::from_bytes(&[1, 2, 3]);
    /// assert!(!non_empty.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Converts the data to a `Vec<u8>`, creating a copy.
    ///
    /// # Examples
    /// ```
    /// # use zewif::Data;
    /// let data = Data::from_bytes(&[1, 2, 3]);
    /// let vec = data.to_vec();
    /// assert_eq!(vec, vec![1, 2, 3]);
    /// ```
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    /// Returns the underlying value as a byte slice.
    pub fn as_slice(&self) -> &[u8] {
        &self.0[..]
    }

    /// Creates a `Data` instance from a byte slice.
    ///
    /// # Examples
    /// ```
    /// # use zewif::Data;
    /// let slice = &[1, 2, 3];
    /// let data = Data::from_slice(slice);
    /// assert_eq!(data.to_vec(), slice);
    /// ```
    pub fn from_slice(data: &[u8]) -> Self {
        Self(data.to_vec())
    }

    /// Creates a `Data` instance from a `Vec<u8>`, taking ownership of the vector.
    ///
    /// This is more efficient than `from_slice` when you already have a vector,
    /// as it avoids making a copy of the data.
    ///
    /// # Examples
    /// ```
    /// # use zewif::Data;
    /// let vec = vec![1, 2, 3];
    /// let data = Data::from_vec(vec.clone());
    /// assert_eq!(data.to_vec(), vec);
    /// ```
    pub fn from_vec(data: Vec<u8>) -> Self {
        Self(data)
    }

    /// Creates a `Data` instance from a hexadecimal string.
    ///
    /// # Errors
    /// Returns an error if the hex string is invalid (contains non-hex characters
    /// or has an odd length).
    ///
    /// # Examples
    /// ```
    /// # use zewif::Data;
    /// let hex = "010203";
    /// let data = Data::from_hex(hex).unwrap();
    /// assert_eq!(data.to_vec(), vec![1, 2, 3]);
    ///
    /// // Invalid hex string
    /// let result = Data::from_hex("01020Z");
    /// assert!(result.is_err());
    /// ```
    pub fn from_hex(hex: &str) -> Result<Self> {
        Ok(Self(hex::decode(hex)?))
    }

    /// Concatenates multiple byte arrays into a single `Data` instance.
    ///
    /// This is useful for combining multiple binary data sources into one.
    ///
    /// # Examples
    /// ```
    /// # use zewif::Data;
    ///
    /// let data1: &[u8] = &[1, 2];
    /// let data2: &[u8] = &[3, 4];
    /// let combined = Data::concat(&[&data1, &data2]);
    ///
    /// assert_eq!(combined.to_vec(), vec![1, 2, 3, 4]);
    /// ```
    pub fn concat(a: &[&dyn AsRef<[u8]>]) -> Self {
        let mut bytes = Vec::new();
        for data in a {
            bytes.extend_from_slice(data.as_ref());
        }
        Self(bytes)
    }
}

impl Default for Data {
    fn default() -> Self {
        Self::new()
    }
}

impl Index<usize> for Data {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Data {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<Range<usize>> for Data {
    type Output = [u8];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl IndexMut<Range<usize>> for Data {
    fn index_mut(&mut self, range: Range<usize>) -> &mut Self::Output {
        &mut self.0[range]
    }
}

impl Index<RangeTo<usize>> for Data {
    type Output = [u8];

    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl IndexMut<RangeTo<usize>> for Data {
    fn index_mut(&mut self, range: RangeTo<usize>) -> &mut Self::Output {
        &mut self.0[range]
    }
}

impl Index<RangeFrom<usize>> for Data {
    type Output = [u8];

    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl IndexMut<RangeFrom<usize>> for Data {
    fn index_mut(&mut self, range: RangeFrom<usize>) -> &mut Self::Output {
        &mut self.0[range]
    }
}

impl Index<RangeFull> for Data {
    type Output = [u8];

    fn index(&self, range: RangeFull) -> &Self::Output {
        &self.0[range]
    }
}

impl IndexMut<RangeFull> for Data {
    fn index_mut(&mut self, range: RangeFull) -> &mut Self::Output {
        &mut self.0[range]
    }
}

impl Index<RangeInclusive<usize>> for Data {
    type Output = [u8];

    fn index(&self, range: RangeInclusive<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl IndexMut<RangeInclusive<usize>> for Data {
    fn index_mut(&mut self, range: RangeInclusive<usize>) -> &mut Self::Output {
        &mut self.0[range]
    }
}

impl Index<RangeToInclusive<usize>> for Data {
    type Output = [u8];

    fn index(&self, range: RangeToInclusive<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl IndexMut<RangeToInclusive<usize>> for Data {
    fn index_mut(
        &mut self,
        range: RangeToInclusive<usize>,
    ) -> &mut Self::Output {
        &mut self.0[range]
    }
}

impl AsRef<[u8]> for Data {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Data<{}>({})", self.len(), hex::encode(self))
    }
}

impl AsRef<Data> for Data {
    fn as_ref(&self) -> &Data {
        self
    }
}

impl From<Data> for Vec<u8> {
    fn from(data: Data) -> Vec<u8> {
        data.to_vec()
    }
}

impl From<&Data> for Vec<u8> {
    fn from(data: &Data) -> Vec<u8> {
        data.to_vec()
    }
}

impl From<Vec<u8>> for Data {
    fn from(data: Vec<u8>) -> Self {
        Self::from_vec(data)
    }
}

impl From<&[u8]> for Data {
    fn from(data: &[u8]) -> Self {
        Self::from_slice(data)
    }
}

impl From<Data> for CBOR {
    fn from(data: Data) -> Self {
        CBOR::to_byte_string(data)
    }
}

impl From<&Data> for CBOR {
    fn from(data: &Data) -> Self {
        CBOR::to_byte_string(data)
    }
}

impl TryFrom<CBOR> for Data {
    type Error = dcbor::Error;

    fn try_from(cbor: CBOR) -> dcbor::Result<Self> {
        let bytes = cbor.try_into_byte_string()?;
        Ok(Data::from_slice(&bytes))
    }
}

impl From<Data> for Envelope {
    fn from(value: Data) -> Self {
        Envelope::new(CBOR::from(value))
    }
}

impl TryFrom<Envelope> for Data {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.extract_subject()
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_cbor_roundtrip, test_envelope_roundtrip};

    use super::Data;

    impl crate::RandomInstance for Data {
        fn random_with_size(size: usize) -> Self
        where
            Self: Sized,
        {
            bc_rand::random_data(size).into()
        }

        fn random() -> Self {
            Self::random_with_size(32)
        }
    }

    test_cbor_roundtrip!(Data);
    test_envelope_roundtrip!(Data);
}
