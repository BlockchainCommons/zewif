use bc_envelope::prelude::*;

use crate::{Amount, Indexed, Memo};

/// Represents a sent output in an Orchard shielded transaction within a Zcash wallet.
///
/// `OrchardSentOutput` stores the plaintext details of an Orchard note that was sent by the
/// wallet, which are not recoverable from the blockchain after transmission. This information
/// enables selective disclosure, allowing a sender to prove they made a payment to a specific
/// shielded address without revealing additional transaction details.
///
/// # Zcash Concept Relation
/// In Zcash's Orchard protocol (introduced in Network Upgrade 5):
///
/// - **Orchard** represents the next-generation shielded protocol after Sapling
/// - **Improved cryptography** using the Pallas elliptic curve (instead of Jubjub in Sapling)
/// - **Enhanced privacy** with design improvements over earlier protocols
/// - **Halo 2 proving system** replaces the previous trusted setup zk-SNARKs
///
/// Each sent output contains components of the Orchard note:
/// - Diversifier: Part of the recipient's shielded address derivation
/// - Public key: The recipient's public key for the transaction
/// - Value: The amount of ZEC transferred
/// - Rho/Psi: Protocol-specific randomness values
/// - Rcm: Random commitment material
///
/// # Data Preservation
/// During wallet migration, Orchard sent output information must be preserved to maintain
/// the ability to generate payment proofs for regulatory compliance, auditing,
/// or other selective disclosure purposes. The sending wallet is the only entity
/// that has this information in plaintext form.
///
/// # Examples
/// ```
/// # use zewif::{orchard::OrchardSentOutput, Blob, Amount};
/// # use zewif::Result;
/// # fn example() -> Result<()> {
/// // Create a new Orchard sent output with all required components
/// let value = Amount::from_u64(10_000_000)?; // 0.1 ZEC
///
/// let sent_output = OrchardSentOutput::from_parts(
///     0,
///     "".to_string(),
///     value,
///     None
/// );
///
/// // Check the value was set correctly
/// let value_zatoshi: i64 = sent_output.value().into();
/// assert_eq!(value_zatoshi, 10_000_000);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct OrchardSentOutput {
    /// The index of the sent output in the transaction.
    index: usize,

    /// The string representation of the address to which the output was sent.
    ///
    /// This should be a Unified address with an Orchard receiver. We preserve the original address
    /// string because in the case of Unified addresses, it is not otherwise possible to
    /// reconstruct the data for receivers other than the Orchard receiver, and as a consequence
    /// the recipient address in a restored wallet would appear different than in the source
    /// wallet.
    recipient_address: String,

    /// The value of ZEC sent in this output, in zatoshis (1 ZEC = 10^8 zatoshis).
    ///
    /// This 64-bit unsigned integer denotes the amount sent, with the same maximum value
    /// constraint as Sapling (2^63 - 1 zatoshis). It is a core component of the note
    /// for tracking and proving the transaction amount.
    value: Amount,

    /// The memo attached to this output, if any.
    memo: Option<Memo>,
}

impl Indexed for OrchardSentOutput {
    fn index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

impl OrchardSentOutput {
    /// Creates a new `OrchardSentOutput` from its constituent parts.
    ///
    /// This constructor creates an Orchard sent output with all required components
    /// for transaction reconstruction and selective disclosure.
    ///
    /// # Arguments
    /// * `value` - The amount of ZEC sent
    ///
    /// # Returns
    /// A new `OrchardSentOutput` instance with the provided values.
    ///
    /// # Examples
    /// ```
    /// # use zewif::{orchard::OrchardSentOutput, Blob, Amount};
    /// # use zewif::Result;
    /// # fn example() -> Result<()> {
    /// let sent_output = OrchardSentOutput::from_parts(
    ///     0,
    ///     "".to_string(),
    ///     Amount::from_u64(5_000_000)?, // 0.05 ZEC
    ///     None
    /// );
    /// # Ok(())
    /// # }
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn from_parts(
        index: usize,
        recipient_address: String,
        value: Amount,
        memo: Option<Memo>,
    ) -> Self {
        Self {
            index,
            recipient_address,
            value,
            memo,
        }
    }

    /// Returns the string representation of the address used in construction of the output.
    ///
    /// This will be a Unified address with an Orchard receiver. We preserve the original address
    /// string because in the case of Unified addresses, it is not otherwise possible to
    /// reconstruct the data for receivers other than the Orchard receiver, and as a consequence
    /// the recipient address in a restored wallet would appear different than in the source
    /// wallet.
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
    /// # use zewif::{orchard::OrchardSentOutput, Blob, Amount};
    /// # use zewif::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let value = Amount::from_u64(15_000_000)?;
    /// # let sent_output = OrchardSentOutput::from_parts(
    /// #     0, "".to_string(), value, None
    /// # );
    /// #
    /// let amount = sent_output.value();
    /// let zats: i64 = amount.into();
    /// assert_eq!(zats, 15_000_000);
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
    /// # use zewif::{orchard::OrchardSentOutput, Blob, Amount};
    /// # use zewif::Result;
    /// #
    /// # fn example() -> Result<()> {
    /// # let mut sent_output = OrchardSentOutput::from_parts(
    /// #     0, "".to_string(), Amount::from_u64(1000)?, None
    /// # );
    /// #
    /// let amount = Amount::from_u64(25_000_000)?; // 0.25 ZEC
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

impl From<OrchardSentOutput> for Envelope {
    fn from(value: OrchardSentOutput) -> Self {
        Envelope::new(value.index)
            .add_type("OrchardSentOutput")
            .add_assertion("recipient_address", value.recipient_address)
            .add_assertion("value", value.value)
            .add_optional_assertion("memo", value.memo)
    }
}

impl TryFrom<Envelope> for OrchardSentOutput {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type_envelope("OrchardSentOutput")?;
        let index = envelope.extract_subject()?;
        let recipient_address = envelope.extract_object_for_predicate("recipient_address")?;
        let value = envelope.extract_object_for_predicate("value")?;
        let memo = envelope.extract_optional_object_for_predicate("memo")?;

        Ok(OrchardSentOutput {
            index,
            recipient_address,
            value,
            memo,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Amount, Memo, UnifiedAddress, test_envelope_roundtrip};

    use super::OrchardSentOutput;

    impl crate::RandomInstance for OrchardSentOutput {
        fn random() -> Self {
            Self {
                index: 0,
                recipient_address: UnifiedAddress::random().address().to_string(),
                value: Amount::random(),
                memo: Some(Memo::random()),
            }
        }
    }

    test_envelope_roundtrip!(OrchardSentOutput);
}
