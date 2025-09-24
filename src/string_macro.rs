/// Creates a new type wrapping a String with common methods and trait
/// implementations.
///
/// The `string!` macro generates a new type that wraps a Rust `String`,
/// automatically implementing common methods and traits. This provides a
/// convenient way to create domain-specific string types with minimal
/// boilerplate.
///
/// # Usage
///
/// ```
/// # use zewif::string;
/// #
/// // Define a type for address labels
/// string!(AddressLabel, "A label for a Zcash address");
///
/// // Use the generated type
/// let label = AddressLabel::from("My Savings".to_string());
/// ```
///
/// # Generated Functionality
///
/// The generated type includes conversions to and from standard Rust string
/// types, as well as implementations for common traits like `Parse`, `Debug`,
/// `Display`, `Clone`, `PartialEq`, `Eq`, and `Hash`.
///
/// In the ZeWIF format, this macro is useful for creating strongly-typed
/// string values such as address labels, wallet notes, purpose strings, and
/// other textual metadata associated with addresses and accounts.
#[macro_export]
macro_rules! string {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        pub struct $name(String);

        impl Clone for $name {
            fn clone(&self) -> Self { Self(self.0.clone()) }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
        }

        impl Eq for $name {}

        impl std::hash::Hash for $name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.0.hash(state);
            }
        }

        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}({:?})", stringify!($name), self.0)
            }
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl Default for $name {
            /// Creates a new empty string instance.
            fn default() -> Self { Self(String::new()) }
        }

        impl From<$name> for String {
            /// Converts this wrapped string type to a standard Rust String.
            fn from(s: $name) -> Self { s.0 }
        }

        impl From<&$name> for String {
            /// Creates a copy of this wrapped string as a standard Rust String.
            fn from(s: &$name) -> Self { s.0.clone() }
        }

        impl From<String> for $name {
            /// Creates a new instance from a standard Rust String.
            fn from(s: String) -> Self { Self(s) }
        }

        impl From<&str> for $name {
            /// Creates a new instance from a string slice.
            fn from(s: &str) -> Self { Self(s.to_string()) }
        }
    };
}
