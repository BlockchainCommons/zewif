use bc_envelope::prelude::*;

use crate::{Amount, Indexed, Memo};

/// Represents a sent output in a Sapling shielded transaction within a Zcash wallet.
///
/// `SaplingSentOutput` stores the plaintext details of a Sapling note that was sent by the
/// wallet, which are not recoverable from the blockchain after transmission. This information
/// enables selective disclosure, allowing a sender to prove they made a payment to a specific
/// shielded address without revealing additional transaction details.
///
/// # Zcash Concept Relation
/// In Zcash's Sapling protocol (activated in October 2018):
///
/// - **Shielded transactions** encrypt transaction details on the blockchain using zk-SNARKs
/// - **Notes** are the fundamental unit of value transfer, similar to UTXOs in transparent transactions
/// - **Sent output information** is stored by the sender's wallet to enable proofs of payment
///
/// Each sent output contains components of the Sapling note:
/// - Diversifier: Part of the recipient's shielded address derivation
/// - Public key: The recipient's public key for the transaction
/// - Value: The amount of ZEC transferred
/// - Rcm: Random commitment material used to construct the note commitment
///
/// # Data Preservation
/// During wallet migration, sent output information must be preserved to maintain
/// the ability to generate payment proofs for regulatory compliance, auditing,
/// or other selective disclosure purposes. The sending wallet is the only entity
/// that has this information in plaintext form.
///
/// # Examples
/// ```
/// # use zewif::{sapling::SaplingSentOutput, Blob, Amount};
/// # use zewif::Result;
/// # fn example() -> Result<()> {
/// // Create a new sent output
/// let mut sent_output = SaplingSentOutput::new();
///
/// let value = Amount::from_u64(5000000)?; // 0.05 ZEC
/// sent_output.set_value(value);
///
/// // Access the components
/// let amount = sent_output.value();
/// let zats: i64 = amount.into();
/// assert_eq!(zats, 5000000);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SaplingSentOutput {
    /// The index of the output in the transaction's Sapling bundle.
    index: usize,

    /// The string representation of the address to which the output was sent.
    ///
    /// This should be either a Sapling address or a Unified address with a Sapling receiver.
    /// We preserve the original address string because in the case of Unified addresses, it is not
    /// otherwise possible to reconstruct the data for receivers other than the Sapling receiver,
    /// and as a consequence the recipient address in a restored wallet would appear different than
    /// in the source wallet.
    recipient_address: String,

    /// The value of ZEC sent in this output, in zatoshis (1 ZEC = 10^8 zatoshis).
    ///
    /// This 64-bit unsigned integer specifies the amount transferred. It is constrained
    /// by the protocol to a maximum value (2^63 - 1 zatoshis), ensuring it fits within
    /// the note's value field for Sapling transactions.
    value: Amount,

    /// The memo attached to this output, if any.
    memo: Option<Memo>,
}

impl Indexed for SaplingSentOutput {
    fn index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

impl SaplingSentOutput {
    /// Creates a new `SaplingSentOutput` with default values.
    ///
    /// This constructor initializes a `SaplingSentOutput` with empty default values
    /// for all fields. In practical use, these values would be set using the setter
    /// methods before the object is used.
    ///
    /// # Returns
    /// A new `SaplingSentOutput` instance with default values.
    ///
    /// # Examples
    /// ```
    /// # use zewif::sapling::SaplingSentOutput;
    /// let sent_output = SaplingSentOutput::new();
    /// ```
    pub fn new() -> Self {
        Self {
            index: 0,
            recipient_address: "".to_string(),
            value: Amount::zero(),
            memo: None,
        }
    }

    /// Creates a new `SaplingSentOutput` from its constituent parts.
    pub fn from_parts(
        index: usize,
        recipient_address: String,
        value: Amount,
        memo: Option<Memo>,
    ) -> Self {
        Self { index, recipient_address, value, memo }
    }

    /// Returns the string representation of the address used in construction of the output.
    ///
    /// This will be either a Sapling address or a Unified address with a Sapling receiver.
    /// We preserve the original address string because in the case of Unified addresses, it is not
    /// otherwise possible to reconstruct the data for receivers other than the Sapling receiver,
    /// and as a consequence the recipient address in a restored wallet would appear different than
    /// in the source wallet.
    pub fn recipient_address(&self) -> &str {
        &self.recipient_address
    }

    /// Sets the string representation of the address used in construction of the output.
    pub fn set_recipient_address(&mut self, recipient_address: String) {
        self.recipient_address = recipient_address;
    }

    /// Returns the value (amount) of ZEC sent in this output.
    ///
    /// This represents the amount of ZEC transferred in this specific note,
    /// measured in zatoshis (1 ZEC = 10^8 zatoshis).
    ///
    /// # Returns
    /// The amount of ZEC as an `Amount`.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{sapling::SaplingSentOutput, Amount};
    /// # use zewif::Result;
    /// # fn example() -> Result<()> {
    /// let mut sent_output = SaplingSentOutput::new();
    /// sent_output.set_value(Amount::from_u64(10_000_000)?); // 0.1 ZEC
    ///
    /// let value = sent_output.value();
    /// let zats: i64 = value.into();
    /// assert_eq!(zats, 10_000_000);
    /// # Ok(())
    /// # }
    /// ```
    pub fn value(&self) -> Amount {
        self.value
    }

    /// Sets the value (amount) of ZEC for this sent output.
    ///
    /// # Arguments
    /// * `value` - The amount of ZEC to set
    ///
    /// # Examples
    /// ```
    /// # use zewif::{sapling::SaplingSentOutput, Amount};
    /// # use zewif::Result;
    /// # fn example() -> Result<()> {
    /// let mut sent_output = SaplingSentOutput::new();
    /// let amount = Amount::from_u64(50_000_000)?; // 0.5 ZEC
    /// sent_output.set_value(amount);
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_value(&mut self, value: Amount) {
        self.value = value;
    }

    /// Returns the memo associated with the output, if known.
    pub fn memo(&self) -> Option<&Memo> {
        self.memo.as_ref()
    }

    /// Sets the memo associated with the output.
    pub fn set_memo(&mut self, memo: Option<Memo>) {
        self.memo = memo;
    }
}

impl Default for SaplingSentOutput {
    fn default() -> Self {
        Self::new()
    }
}

impl From<SaplingSentOutput> for Envelope {
    fn from(value: SaplingSentOutput) -> Self {
        Envelope::new(value.index)
            .add_type("SaplingSentOutput")
            .add_assertion("recipient_address", value.recipient_address)
            .add_assertion("value", value.value)
            .add_optional_assertion("memo", value.memo)
    }
}

impl TryFrom<Envelope> for SaplingSentOutput {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("SaplingSentOutput")?;
        let index = envelope.extract_subject()?;
        let recipient_address =
            envelope.extract_object_for_predicate("recipient_address")?;
        let value = envelope.extract_object_for_predicate("value")?;
        let memo = envelope.extract_optional_object_for_predicate("memo")?;

        Ok(SaplingSentOutput { index, recipient_address, value, memo })
    }
}

#[cfg(test)]
mod tests {
    use super::SaplingSentOutput;
    use crate::{Amount, Memo, test_envelope_roundtrip};

    impl crate::RandomInstance for SaplingSentOutput {
        fn random() -> Self {
            Self {
                index: 0,
                recipient_address: String::random(),
                value: Amount::random(),
                memo: Some(Memo::random()),
            }
        }
    }

    test_envelope_roundtrip!(SaplingSentOutput);
}
