/// Creates a new type wrapping a variable-size byte array with common methods and trait implementations.
///
/// The `data!` macro generates a new type that wraps a [`Data`](crate::Data) container,
/// automatically implementing common methods and traits. This provides a convenient way to
/// create domain-specific types for variable-length binary data with minimal boilerplate.
///
/// # Usage
///
/// ```
/// # use zewif::data;
/// #
/// // Define a type for variable-length script data
/// data!(ScriptData, "A variable-length script for Zcash transactions.");
///
/// // Use the generated type
/// let script = ScriptData::new(vec![0xAA, 0xBB, 0xCC]);
/// ```
///
/// # Generated Functionality
///
/// The generated type includes methods for creation, conversion, and inspection,
/// as well as implementations for common traits like `Parse`, `Debug`, `Clone`,
/// and various conversion traits to and from byte collections.
///
/// This macro is especially useful for creating strong types around Zcash protocol
/// elements that have variable lengths, such as encrypted memos, scripts, and
/// other dynamically-sized data.
#[macro_export]
macro_rules! data {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        pub struct $name($crate::Data);

        impl $name {
            /// Creates a new instance from a vector of bytes, taking ownership.
            ///
            /// This is the primary constructor when you have a `Vec<u8>` available.
            pub fn new(data: Vec<u8>) -> Self {
                Self($crate::Data::from_vec(data))
            }

            /// Returns the number of bytes in this data container.
            pub fn len(&self) -> usize {
                self.0.len()
            }

            /// Returns `true` if this data container is empty (contains no bytes).
            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }

            /// Converts this data to a `Vec<u8>`, creating a copy.
            pub fn to_vec(&self) -> Vec<u8> {
                self.0.to_vec()
            }

            /// Creates an instance from a slice of bytes.
            pub fn from_slice(data: &[u8]) -> Self {
                Self($crate::Data::from_slice(data))
            }

            /// Creates an instance from a `Vec<u8>`, taking ownership of the vector.
            pub fn from_vec(data: Vec<u8>) -> Self {
                Self($crate::Data::from_vec(data))
            }

            /// Creates an instance from a hexadecimal string.
            ///
            /// # Panics
            /// Panics if the hex string cannot be decoded.
            pub fn from_hex(hex: &str) -> Self {
                Self($crate::Data::from_hex(hex).unwrap())
            }
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                Self(self.0.clone())
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}({:?})", stringify!($name), self.0)
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self($crate::Data::default())
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }
        }

        impl Eq for $name {}

        impl AsRef<[u8]> for $name {
            fn as_ref(&self) -> &[u8] {
                self.0.as_ref()
            }
        }

        impl From<$name> for Vec<u8> {
            fn from(data: $name) -> Vec<u8> {
                data.to_vec()
            }
        }

        impl From<&$name> for Vec<u8> {
            fn from(data: &$name) -> Vec<u8> {
                data.to_vec()
            }
        }

        impl From<Vec<u8>> for $name {
            fn from(data: Vec<u8>) -> Self {
                Self::from_vec(data)
            }
        }

        impl From<&[u8]> for $name {
            fn from(data: &[u8]) -> Self {
                Self::from_slice(data)
            }
        }

        impl From<$name> for bc_envelope::prelude::CBOR {
            fn from(data: $name) -> Self {
                bc_envelope::prelude::CBOR::to_byte_string(data.0)
            }
        }

        impl From<&$name> for bc_envelope::prelude::CBOR {
            fn from(data: &$name) -> Self {
                bc_envelope::prelude::CBOR::to_byte_string(data.0.clone())
            }
        }

        impl TryFrom<bc_envelope::prelude::CBOR> for $name {
            type Error = dcbor::Error;

            fn try_from(
                cbor: bc_envelope::prelude::CBOR,
            ) -> Result<Self, Self::Error> {
                let bytes = cbor.try_into_byte_string()?;
                Ok(Self::from_slice(&bytes))
            }
        }

        #[cfg(test)]
        impl $crate::RandomInstance for $name {
            fn random() -> Self {
                Self($crate::Data::random())
            }
        }
    };
}
