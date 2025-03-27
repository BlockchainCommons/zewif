#![allow(dead_code)]

use bc_components::ARID;

/// A trait for objects that have a unique identifier within the wallet
/// interchange format.
pub trait Identifiable {
    fn id(&self) -> &ARID;
}
