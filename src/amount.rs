use std::{
    iter::Sum,
    ops::{Add, Mul, Neg, Sub},
};

use crate::error::{Error, Result};
use bc_envelope::prelude::*;

use crate::format_signed_zats_as_zec;

/// Number of zatoshis (zats) in 1 ZEC
pub const COIN: u64 = 1_0000_0000;
/// Maximum possible ZEC supply in zatoshis (21 million ZEC)
pub const MAX_MONEY: u64 = 21_000_000 * COIN;
/// Maximum balance as a signed value
pub const MAX_BALANCE: i64 = MAX_MONEY as i64;

/// A type-safe representation of a ZCash amount in zatoshis (zats).
///
/// `Amount` represents a monetary value in the Zcash cryptocurrency, stored
/// internally as a signed 64-bit integer count of zatoshis. One ZEC equals
/// 100,000,000 zatoshis (1 ZEC = 10^8 zats), similar to Bitcoin's satoshis.
///
/// The signed representation allows for representing both positive amounts
/// (payments received) and negative amounts (payments sent) in transaction
/// and balance calculations.
///
/// # Zcash Concept Relation
/// In Zcash, monetary values are represented in two units:
/// - ZEC: The main unit of currency (analogous to dollars)
/// - zatoshis (zats): The smallest indivisible unit (analogous to cents)
///
/// Amount enforces the protocol limit of 21 million total ZEC, preventing
/// overflow or underflow in calculations with proper error handling.
///
/// # Data Preservation
/// The `Amount` type preserves the exact zatoshi values from wallet data,
/// maintaining precise balances and transaction amounts during wallet migration.
/// When displayed, values are formatted as ZEC with decimal places.
///
/// # Examples
/// ```
/// # use zewif::Amount;
/// # use zewif::Result;
/// #
/// # fn example() -> Result<()> {
/// // Create an amount of 1.5 ZEC (150,000,000 zatoshis)
/// let amount = Amount::from_u64(150_000_000)?;
///
/// // Check if the amount is positive
/// assert!(amount.is_positive());
///
/// // Convert to raw zatoshi value
/// let zats: i64 = amount.into();
/// assert_eq!(zats, 150_000_000);
/// # Ok(())
/// # }
/// ```
/// FIXME: Amounts in the zewif format should never be negative; negative values are only used
/// transiently in the protocol.
#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct Amount(i64);

impl std::fmt::Debug for Amount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Amount({})", format_signed_zats_as_zec(self.0))
    }
}

impl Amount {
    /// Returns a zero-valued Amount.
    pub const fn zero() -> Self {
        Amount(0)
    }

    /// Creates a constant Amount from an i64.
    ///
    /// Panics: if the amount is outside the range `{-MAX_BALANCE..MAX_BALANCE}`.
    pub const fn const_from_i64(amount: i64) -> Self {
        assert!(-MAX_BALANCE <= amount && amount <= MAX_BALANCE); // contains is not const
        Amount(amount)
    }

    /// Creates a constant Amount from a u64.
    ///
    /// Panics: if the amount is outside the range `{0..MAX_BALANCE}`.
    pub const fn const_from_u64(amount: u64) -> Self {
        assert!(amount <= MAX_MONEY); // contains is not const
        Amount(amount as i64)
    }

    /// Creates an Amount from an i64.
    ///
    /// Returns an error if the amount is outside the range `{-MAX_BALANCE..MAX_BALANCE}`.
    pub fn from_i64(amount: i64) -> Result<Self> {
        if (-MAX_BALANCE..=MAX_BALANCE).contains(&amount) {
            Ok(Amount(amount))
        } else if amount < -MAX_BALANCE {
            Err(Error::AmountUnderflow(amount as u64))
        } else {
            Err(Error::AmountOverflow(amount as u64))
        }
    }

    /// Creates a non-negative Amount from an i64.
    ///
    /// Returns an error if the amount is outside the range `{0..MAX_BALANCE}`.
    pub fn from_nonnegative_i64(amount: i64) -> Result<Self> {
        if (0..=MAX_BALANCE).contains(&amount) {
            Ok(Amount(amount))
        } else if amount < 0 {
            Err(Error::AmountUnderflow(amount as u64))
        } else {
            Err(Error::AmountOverflow(amount as u64))
        }
    }

    /// Creates an Amount from a u64.
    ///
    /// Returns an error if the amount is outside the range `{0..MAX_MONEY}`.
    pub fn from_u64(amount: u64) -> Result<Self> {
        if amount <= MAX_MONEY {
            Ok(Amount(amount as i64))
        } else {
            Err(Error::AmountOverflow(amount))
        }
    }

    /// Reads an Amount from a signed 64-bit little-endian integer.
    ///
    /// Returns an error if the amount is outside the range `{-MAX_BALANCE..MAX_BALANCE}`.
    pub fn from_i64_le_bytes(bytes: [u8; 8]) -> Result<Self> {
        let amount = i64::from_le_bytes(bytes);
        Amount::from_i64(amount)
    }

    /// Reads a non-negative Amount from a signed 64-bit little-endian integer.
    ///
    /// Returns an error if the amount is outside the range `{0..MAX_BALANCE}`.
    pub fn from_nonnegative_i64_le_bytes(bytes: [u8; 8]) -> Result<Self> {
        let amount = i64::from_le_bytes(bytes);
        Amount::from_nonnegative_i64(amount)
    }

    /// Reads an Amount from an unsigned 64-bit little-endian integer.
    ///
    /// Returns an error if the amount is outside the range `{0..MAX_BALANCE}`.
    pub fn from_u64_le_bytes(bytes: [u8; 8]) -> Result<Self> {
        let amount = u64::from_le_bytes(bytes);
        Amount::from_u64(amount)
    }

    /// Returns the Amount encoded as a signed 64-bit little-endian integer.
    pub fn to_i64_le_bytes(self) -> [u8; 8] {
        self.0.to_le_bytes()
    }

    /// Returns `true` if `self` is positive and `false` if the Amount is zero or
    /// negative.
    pub const fn is_positive(self) -> bool {
        self.0.is_positive()
    }

    /// Returns `true` if `self` is negative and `false` if the Amount is zero or
    /// positive.
    pub const fn is_negative(self) -> bool {
        self.0.is_negative()
    }

    /// Sums a collection of Amount values with overflow checking.
    ///
    /// This helper method safely adds a collection of Amounts, returning None if
    /// any intermediate calculation would exceed the valid Amount range.
    ///
    /// # Arguments
    /// * `values` - An iterable collection of Amount values to sum
    ///
    /// # Returns
    /// * `Some(Amount)` - The sum if all operations were successful
    /// * `None` - If any intermediate sum would exceed MAX_BALANCE
    ///
    /// # Examples
    /// ```
    /// # use zewif::Amount;
    /// # use zewif::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// // Sum several ZEC amounts
    /// let amounts = vec![
    ///     Amount::from_u64(100_000_000)?, // 1 ZEC
    ///     Amount::from_u64(50_000_000)?,  // 0.5 ZEC
    ///     Amount::from_u64(25_000_000)?,  // 0.25 ZEC
    /// ];
    ///
    /// let total = Amount::sum(amounts).unwrap();
    /// let total_zats: i64 = total.into();
    /// assert_eq!(total_zats, 175_000_000); // 1.75 ZEC
    /// # Ok(())
    /// # }
    /// ```
    pub fn sum<I: IntoIterator<Item = Amount>>(values: I) -> Option<Amount> {
        let mut result = Amount::zero();
        for value in values {
            result = (result + value)?;
        }
        Some(result)
    }
}

/// Converts an i64 into an Amount, with range checking
impl TryFrom<i64> for Amount {
    type Error = Error;

    fn try_from(value: i64) -> Result<Self> {
        Amount::from_i64(value)
    }
}

/// Extracts the raw i64 zatoshi value from an Amount
impl From<Amount> for i64 {
    fn from(amount: Amount) -> i64 {
        amount.0
    }
}

/// Extracts the raw i64 zatoshi value from an Amount reference
impl From<&Amount> for i64 {
    fn from(amount: &Amount) -> i64 {
        amount.0
    }
}

/// Converts an Amount to u64, ensuring the value is non-negative
impl TryFrom<Amount> for u64 {
    type Error = Error;

    fn try_from(value: Amount) -> crate::error::Result<Self> {
        value
            .0
            .try_into()
            .map_err(|_| Error::AmountUnderflow(value.0 as u64))
    }
}

/// Adds two Amounts, checking for overflow/underflow
impl Add<Amount> for Amount {
    type Output = Option<Amount>;

    fn add(self, rhs: Amount) -> Option<Amount> {
        Amount::from_i64(self.0 + rhs.0).ok()
    }
}

/// Adds an Amount to an `Option<Amount>`, propagating None
impl Add<Amount> for Option<Amount> {
    type Output = Self;

    fn add(self, rhs: Amount) -> Option<Amount> {
        self.and_then(|lhs| lhs + rhs)
    }
}

/// Subtracts one Amount from another, checking for overflow/underflow
impl Sub<Amount> for Amount {
    type Output = Option<Amount>;

    fn sub(self, rhs: Amount) -> Option<Amount> {
        Amount::from_i64(self.0 - rhs.0).ok()
    }
}

/// Subtracts an Amount from an `Option<Amount>`, propagating None
impl Sub<Amount> for Option<Amount> {
    type Output = Self;

    fn sub(self, rhs: Amount) -> Option<Amount> {
        self.and_then(|lhs| lhs - rhs)
    }
}

/// Implements std::iter::Sum for Amount with overflow checking
impl Sum<Amount> for Option<Amount> {
    fn sum<I: Iterator<Item = Amount>>(mut iter: I) -> Self {
        iter.try_fold(Amount::zero(), |acc, a| acc + a)
    }
}

/// Implements std::iter::Sum for Amount references with overflow checking
impl<'a> Sum<&'a Amount> for Option<Amount> {
    fn sum<I: Iterator<Item = &'a Amount>>(mut iter: I) -> Self {
        iter.try_fold(Amount::zero(), |acc, a| acc + *a)
    }
}

/// Negates an Amount, flipping its sign
impl Neg for Amount {
    type Output = Self;

    fn neg(self) -> Self {
        Amount(-self.0)
    }
}

/// Multiplies an Amount by a usize factor, checking for overflow/underflow
impl Mul<usize> for Amount {
    type Output = Option<Amount>;

    fn mul(self, rhs: usize) -> Option<Amount> {
        let rhs: i64 = rhs.try_into().ok()?;
        self.0
            .checked_mul(rhs)
            .and_then(|i| Amount::try_from(i).ok())
    }
}

impl From<Amount> for CBOR {
    fn from(value: Amount) -> Self {
        CBOR::from(value.0)
    }
}

impl From<&Amount> for CBOR {
    fn from(value: &Amount) -> Self {
        CBOR::from(value.0)
    }
}

impl TryFrom<CBOR> for Amount {
    type Error = dcbor::Error;

    fn try_from(cbor: CBOR) -> dcbor::Result<Self> {
        let value = i64::try_from(cbor)?;
        Ok(Amount::try_from(value)?)
    }
}

impl From<Amount> for Envelope {
    fn from(value: Amount) -> Self {
        Envelope::new(CBOR::from(value))
    }
}

impl TryFrom<Envelope> for Amount {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.extract_subject()
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_cbor_roundtrip, test_envelope_roundtrip};

    use super::{Amount, MAX_BALANCE};

    impl crate::RandomInstance for Amount {
        fn random() -> Self {
            let mut rng = bc_rand::thread_rng();
            let value =
                rand::Rng::gen_range(&mut rng, -MAX_BALANCE..=MAX_BALANCE);
            Self(value)
        }
    }

    test_cbor_roundtrip!(Amount);
    test_envelope_roundtrip!(Amount);
}
