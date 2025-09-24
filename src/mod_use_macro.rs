/// A macro that simplifies module declarations and re-exports in a single
/// statement.
///
/// The `mod_use!` macro provides a convenient way to declare modules and
/// re-export their contents in a single operation. This greatly reduces
/// boilerplate code when organizing a codebase with many related modules,
/// especially when those modules are intended to be made available through
/// their parent module.
///
/// # Zcash Concept Relation
/// In the ZeWIF codebase, many Zcash protocol elements are organized into
/// logical groupings, such as transaction components, key types, and
/// protocol-specific structures (Sapling, Sprout, Orchard). The `mod_use!`
/// macro helps maintain clean organization while ensuring proper exports.
///
/// # Examples
/// ```ignore
/// # // This example is in a documentation comment, so we don't need to import macro
/// # mod example {
/// use crate::mod_use;
///
/// // Using the macro to define and export three modules
/// mod_use!(
///     transaction_components,
///     key_types,
///     address_formats
/// );
/// # }
/// ```
///
/// This expands to:
/// ```ignore
/// # mod example {
/// mod transaction_components;
/// pub use transaction_components::*;
///
/// mod key_types;
/// pub use key_types::*;
///
/// mod address_formats;
/// pub use address_formats::*;
/// # }
/// ```
#[macro_export]
macro_rules! mod_use {
    ($($name:ident),* $(,)?) => {
        $(
            mod $name; pub use $name::*;
        )*
    };
}
