use crate::error::{Error, Result};
use bc_envelope::prelude::*;
use std::{
    fmt,
    ops::{
        Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
    },
};

/// A fixed-size byte array wrapper for safely handling binary data of known length.
///
/// `Blob<N>` represents an immutable, fixed-size array of bytes that provides
/// safe access and manipulation. It serves as a foundation for many domain-specific
/// types in ZeWIF that need to represent fixed-length binary data, such as:
///
/// - Transaction IDs (32 bytes)
/// - Cryptographic keys (various lengths)
/// - Diversifier indices (11 bytes)
/// - Hashes and other cryptographic values
///
/// # Zcash Concept Relation
/// In Zcash, many protocol elements are represented as fixed-size binary values with
/// specific encodings. `Blob<N>` provides a type-safe way to handle these values
/// and ensures they maintain their correct size throughout operations.
///
/// # Data Preservation
/// `Blob<N>` preserves raw binary data exactly as it appears in wallet files,
/// ensuring that cryptographic material and identifiers maintain their exact
/// binary representation throughout the migration process.
///
/// # Examples
///
/// ## Construction
/// ```
/// # use zewif::Blob;
/// #
/// // Create a blob with 32 bytes (common for transaction hashes)
/// let data = [0u8; 32];
/// let blob = Blob::new(data);
///
/// // Access data with index operations
/// assert_eq!(blob[0], 0);
///
/// // Convert to hex for display
/// let hex_string = hex::encode(blob.as_slice());
/// ```
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Blob<const N: usize>([u8; N]);

impl<const N: usize> Blob<N> {
    /// Creates a new `Blob` from a fixed-size byte array.
    ///
    /// This is the primary constructor for `Blob<N>` when you have an
    /// exact-sized array available.
    ///
    /// # Examples
    /// ```
    /// # use zewif::Blob;
    /// let data = [1, 2, 3, 4];
    /// let blob = Blob::new(data);
    /// ```
    pub fn new(data: [u8; N]) -> Self {
        Self(data)
    }

    /// Returns the length of the blob in bytes.
    ///
    /// This will always return the same value (N) for a given `Blob<N>` type.
    ///
    /// # Examples
    /// ```
    /// # use zewif::Blob;
    /// let blob = Blob::<32>::default();
    /// assert_eq!(blob.len(), 32);
    /// ```
    pub fn len(&self) -> usize {
        N
    }

    /// Returns `true` if the blob contains no bytes (N = 0).
    ///
    /// Note: For most practical uses of `Blob<N>`, this will always return `false`
    /// as N is typically greater than 0.
    ///
    /// # Examples
    /// ```
    /// # use zewif::Blob;
    /// let blob = Blob::<32>::default();
    /// assert!(!blob.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        N == 0
    }

    /// Converts the blob to a `Vec<u8>`, creating a copy of the data.
    ///
    /// # Examples
    /// ```
    /// # use zewif::Blob;
    /// let blob = Blob::<4>::new([1, 2, 3, 4]);
    /// let vec = blob.to_vec();
    /// assert_eq!(vec, vec![1, 2, 3, 4]);
    /// ```
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    /// Returns a reference to the underlying byte array as a slice.
    ///
    /// This is useful when you need to pass the blob's contents to functions
    /// that accept slices.
    ///
    /// # Examples
    /// ```
    /// # use zewif::Blob;
    ///
    /// let blob = Blob::<4>::new([1, 2, 3, 4]);
    /// let slice = blob.as_slice();
    /// assert_eq!(slice, &[1, 2, 3, 4]);
    /// ```
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    /// exposes the underlying byte array.
    pub fn as_bytes(&self) -> &[u8; N] {
        &self.0
    }

    /// Creates a `Blob` from a slice of bytes.
    ///
    /// # Errors
    /// Returns an error if the slice's length doesn't match the expected size N.
    ///
    /// # Examples
    /// ```
    /// # use zewif::Blob;
    ///
    /// let slice = &[1, 2, 3, 4];
    /// let blob = Blob::<4>::from_slice(slice).unwrap();
    /// assert_eq!(blob.as_slice(), slice);
    ///
    /// // This would fail:
    /// let result = Blob::<5>::from_slice(slice);
    /// assert!(result.is_err());
    /// ```
    pub fn from_slice(data: &[u8]) -> Result<Self> {
        Ok(Self(<[u8; N]>::try_from(data)?))
    }

    /// Creates a `Blob` from a `Vec<u8>`.
    ///
    /// # Errors
    /// Returns an error if the vector's length doesn't match the expected size N.
    ///
    /// # Examples
    /// ```
    /// # use zewif::Blob;
    ///
    /// let vec = vec![1, 2, 3, 4];
    /// let blob = Blob::<4>::from_vec(vec.clone()).unwrap();
    /// assert_eq!(blob.to_vec(), vec);
    /// ```
    pub fn from_vec(data: Vec<u8>) -> Result<Self> {
        Self::from_slice(&data)
    }

    /// Parses a `Blob` from a hexadecimal string.
    ///
    /// # Examples
    /// ```
    /// # use zewif::Blob;
    ///
    /// let hex = "01020304";
    /// let blob = Blob::<4>::from_hex(hex).unwrap();
    /// assert_eq!(blob.as_slice(), &[1, 2, 3, 4]);
    /// ```
    pub fn from_hex(hex: &str) -> Result<Self> {
        let data = hex::decode(hex)?;
        let data_len = data.len();
        Self::from_vec(data).map_err(|_| Error::HexLengthMismatch {
            expected: N,
            actual: data_len,
        })
    }
}

impl<const N: usize> Default for Blob<N> {
    fn default() -> Self {
        Self([0u8; N])
    }
}

impl<const N: usize> Index<usize> for Blob<N> {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for Blob<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const N: usize> Index<Range<usize>> for Blob<N> {
    type Output = [u8];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl<const N: usize> Index<RangeTo<usize>> for Blob<N> {
    type Output = [u8];

    fn index(&self, range: RangeTo<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl<const N: usize> Index<RangeFrom<usize>> for Blob<N> {
    type Output = [u8];

    fn index(&self, range: RangeFrom<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl<const N: usize> Index<RangeFull> for Blob<N> {
    type Output = [u8];

    fn index(&self, range: RangeFull) -> &Self::Output {
        &self.0[range]
    }
}

impl<const N: usize> Index<RangeInclusive<usize>> for Blob<N> {
    type Output = [u8];

    fn index(&self, range: RangeInclusive<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl<const N: usize> Index<RangeToInclusive<usize>> for Blob<N> {
    type Output = [u8];

    fn index(&self, range: RangeToInclusive<usize>) -> &Self::Output {
        &self.0[range]
    }
}

impl<const N: usize> From<Blob<N>> for [u8; N] {
    fn from(blob: Blob<N>) -> Self {
        blob.0
    }
}

impl<const N: usize> AsRef<[u8]> for Blob<N> {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl<const N: usize> fmt::Debug for Blob<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Blob<{}>({})", N, hex::encode(self.0))
    }
}

impl<const N: usize> fmt::Display for Blob<N> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl<const N: usize> From<Blob<N>> for Vec<u8> {
    fn from(blob: Blob<N>) -> Vec<u8> {
        blob.to_vec()
    }
}

impl<const N: usize> From<&Blob<N>> for Vec<u8> {
    fn from(blob: &Blob<N>) -> Vec<u8> {
        blob.to_vec()
    }
}

impl<const N: usize> From<Vec<u8>> for Blob<N> {
    fn from(data: Vec<u8>) -> Self {
        Self::from_vec(data).unwrap()
    }
}

impl<const N: usize> From<&[u8]> for Blob<N> {
    fn from(data: &[u8]) -> Self {
        Self::from_vec(data.to_vec()).unwrap()
    }
}

impl<const N: usize> From<&[u8; N]> for Blob<N> {
    fn from(data: &[u8; N]) -> Self {
        Self::from_vec(data.to_vec()).unwrap()
    }
}

/// Type alias for Blob<20>
pub type Blob20 = Blob<20>;
impl Copy for Blob20 {}

/// Type alias for Blob<32>
pub type Blob32 = Blob<32>;
impl Copy for Blob32 {}

/// Type alias for Blob<64>
pub type Blob64 = Blob<64>;

impl<const N: usize> From<Blob<N>> for CBOR {
    fn from(data: Blob<N>) -> Self {
        CBOR::to_byte_string(data)
    }
}

impl<const N: usize> From<&Blob<N>> for CBOR {
    fn from(data: &Blob<N>) -> Self {
        CBOR::to_byte_string(data)
    }
}

impl<const N: usize> TryFrom<CBOR> for Blob<N> {
    type Error = dcbor::Error;

    fn try_from(cbor: CBOR) -> dcbor::Result<Self> {
        let bytes = cbor.try_into_byte_string()?;
        let blob =
            Blob::from_slice(&bytes).map_err(|e| dcbor::Error::msg(format!("Blob: {e}")))?;
        Ok(blob)
    }
}

impl<const N: usize> From<Blob<N>> for Envelope {
    fn from(value: Blob<N>) -> Self {
        Envelope::new(CBOR::from(value))
    }
}

impl<const N: usize> TryFrom<Envelope> for Blob<N> {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.extract_subject()
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_cbor_roundtrip, test_envelope_roundtrip};

    use super::{Blob, Blob32};

    impl<const N: usize> crate::RandomInstance for Blob<N> {
        fn random() -> Self {
            let mut rng = bc_rand::thread_rng();
            Self(bc_rand::rng_random_array(&mut rng))
        }
    }

    test_cbor_roundtrip!(Blob32);
    test_envelope_roundtrip!(Blob32);
}
