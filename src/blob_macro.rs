/// Creates a new type wrapping a fixed-size byte array with common methods and
/// trait implementations.
///
/// The `blob!` macro generates a new type that wraps a [`Blob<N>`](crate::Blob)
/// of the specified size, automatically implementing common methods and traits.
/// This provides a convenient way to create domain-specific types for
/// fixed-size binary data with minimal boilerplate.
///
/// # Usage
///
/// ```
/// # use zewif::blob;
/// #
/// blob!(TxId, 32, "A transaction identifier as a 32-byte hash");
/// ```
///
/// # Generated Functionality
///
/// The generated type includes methods for creation, conversion, and
/// inspection, as well as implementations for common traits like `Parse`,
/// `Debug`, `Clone`, and various conversion traits to and from byte
/// collections.
///
/// The macro adds type safety and domain-specific semantics to otherwise
/// generic byte array data, particularly for cryptographic values used in the
/// Zcash protocol.
#[macro_export]
macro_rules! blob {
    ($name:ident, $size:expr, $doc:expr) => {
        #[doc = $doc]
        pub struct $name([u8; $size]);

        impl $name {
            /// Creates a new instance from a fixed-size byte array.
            ///
            /// This is the primary constructor when you have an exact-sized
            /// array available.
            pub fn new(data: [u8; $size]) -> Self { Self(data) }

            /// Returns the length of this blob in bytes.
            ///
            /// This will always return `$size` for this type.
            pub fn len(&self) -> usize { $size }

            /// Returns `true` if this blob contains no bytes.
            ///
            /// This will always return `false` for this type (unless `$size` is
            /// 0).
            pub fn is_empty(&self) -> bool { $size != 0 }

            /// Converts this blob to a `Vec<u8>`, creating a copy of the data.
            pub fn to_vec(&self) -> Vec<u8> { self.0.to_vec() }

            /// Exposes the underlying byte array as a slice.
            pub fn as_slice(&self) -> &[u8] { &self.0 }

            /// Exposes the underlying byte array.
            pub fn as_bytes(&self) -> &[u8; $size] { &self.0 }

            /// Creates an instance from a slice of bytes.
            ///
            /// # Errors
            /// Returns an error if the slice's length doesn't match the
            /// expected size ($size).
            pub fn from_slice(
                data: &[u8],
            ) -> Result<Self, std::array::TryFromSliceError> {
                Ok(Self(<[u8; $size]>::try_from(data)?))
            }

            /// Creates an instance from a `Vec<u8>`.
            ///
            /// # Errors
            /// Returns an error if the vector's length doesn't match the
            /// expected size ($size).
            pub fn from_vec(
                data: Vec<u8>,
            ) -> Result<Self, std::array::TryFromSliceError> {
                Ok(Self(<[u8; $size]>::try_from(&data[..])?))
            }

            /// Parses an instance from a hex string.
            pub fn from_hex(hex: &str) -> $crate::Result<Self> {
                let data = hex::decode(hex)?;
                let data_len = data.len();
                Self::from_vec(data).map_err(|_| {
                    $crate::Error::HexLengthMismatch {
                        expected: $size,
                        actual: data_len,
                    }
                })
            }

            /// Parses an instance from a hex string in reversed byte order,
            /// such as is used for transaction identifiers and block
            /// hashes.
            pub fn from_reversed_hex(hex: &str) -> $crate::Result<Self> {
                let mut data = hex::decode(hex)?;
                let data_len = data.len();
                data.reverse();
                Self::from_vec(data).map_err(|_| {
                    $crate::Error::HexLengthMismatch {
                        expected: $size,
                        actual: data_len,
                    }
                })
            }

            /// Formats the bytes of this object as a hex string.
            pub fn to_hex(&self) -> String { hex::encode(self.0) }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool { self.0.eq(&other.0) }
        }

        impl Eq for $name {}

        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state)
            }
        }

        impl Clone for $name {
            #[allow(clippy::non_canonical_clone_impl)]
            fn clone(&self) -> Self { Self(self.0.clone()) }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}({})", stringify!($name), hex::encode(self.0))
            }
        }

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &[u8] { &self.0[..] }
        }

        impl From<$name> for Vec<u8> {
            fn from(blob: $name) -> Vec<u8> { blob.to_vec() }
        }

        impl From<&$name> for Vec<u8> {
            fn from(blob: &$name) -> Vec<u8> { blob.to_vec() }
        }

        impl From<Vec<u8>> for $name {
            fn from(data: Vec<u8>) -> Self { Self::from_vec(data).unwrap() }
        }

        impl From<&[u8]> for $name {
            fn from(data: &[u8]) -> Self { Self::from_slice(data).unwrap() }
        }

        impl From<$name> for bc_envelope::prelude::CBOR {
            fn from(data: $name) -> Self {
                bc_envelope::prelude::CBOR::to_byte_string(data.0)
            }
        }

        impl From<&$name> for bc_envelope::prelude::CBOR {
            fn from(data: &$name) -> Self {
                bc_envelope::prelude::CBOR::to_byte_string(data.0)
            }
        }

        impl TryFrom<bc_envelope::prelude::CBOR> for $name {
            type Error = dcbor::Error;

            fn try_from(
                cbor: bc_envelope::prelude::CBOR,
            ) -> Result<Self, Self::Error> {
                let bytes = cbor.try_into_byte_string()?;
                Self::from_slice(&bytes).map_err(|_| {
                    dcbor::Error::msg(format!(
                        "slice length invalid; expected {} bytes, got {}",
                        $size,
                        bytes.len()
                    ))
                })
            }
        }

        #[cfg(test)]
        impl $crate::RandomInstance for $name {
            fn random() -> Self {
                let mut rng = bc_rand::thread_rng();
                Self(bc_rand::rng_random_array(&mut rng))
            }
        }
    };
}
