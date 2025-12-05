use bc_envelope::prelude::*;
use std::collections::HashSet;

use crate::{
    Address, BlockHash, BlockHeight, Indexed, NoQuotesDebugOption, TxId,
    envelope_indexed_objects_for_predicate, orchard::OrchardSentOutput, sapling::SaplingSentOutput,
};

/// A logical grouping of addresses and transaction history within a wallet.
///
/// `Account` represents a distinct subdivision of wallet functionality, similar to how
/// bank accounts work within a single banking relationship. Each account can have its own set of
/// addresses, relevant transactions, and sent output information that cannot be recovered
/// from the blockchain.
///
/// # Zcash Concept Relation
///
/// In Zcash's hierarchical deterministic (HD) wallet structure:
///
/// - **Account Subdivision**: Follows the BIP-44/ZIP-32 HD hierarchy model (m/44'/133'/account'/*),
///   where each account represents a separate logical collection of funds
/// - **Isolation**: Different accounts provide isolation for different purposes or users
/// - **ZIP-32 Integration**: When available, accounts may include ZIP-32 account indexes to
///   maintain the hierarchical derivation structure
///
/// # Data Preservation
///
/// During wallet migration, the following account data must be preserved:
///
/// - **Identity**: The unique account identifier and human-readable name
/// - **Addresses**: All addresses associated with the account, including metadata
/// - **Transaction Relationships**: Which transactions are relevant to this account
/// - **Sent Output Information**: Data that can't be recovered from the blockchain,
///   such as outgoing transaction metadata
/// - **Extended Key Information**: HD wallet derivation path information when available
///
/// # Examples
/// ```no_run
/// # use zewif::{Account, Address, TxId, ProtocolAddress, transparent};
/// #
/// // Create a new account
/// let mut account = Account::new();
///
/// // Set account properties
/// account.set_name("Savings");
/// account.set_zip32_account_id(0);
///
/// // In practice, you would have real addresses and transaction IDs
/// // Create a transparent address (as an example)
/// let t_addr = transparent::Address::new("t1ExampleAddress".to_string());
/// let address = Address::new(ProtocolAddress::Transparent(t_addr));
/// let txid = TxId::from_bytes([0u8; 32]);
///
/// // Add the address and transaction to the account
/// account.add_address(address);
/// account.add_relevant_transaction(txid);
/// ```
#[derive(Clone, PartialEq)]
pub struct Account {
    index: usize,

    // User-defined, may not be unique.
    name: String,

    // The birthday height of the account, if known.
    //
    // This is the minimum block height at which the restoring wallet should trial-decrypt
    // transactions to find shielded inputs.
    birthday_height: Option<BlockHeight>,

    // The hash of the birthday block, if known.
    //
    // If the wallet's birthday height is within 100 blocks of the export height for the overall
    // [`Zewif`] value containing the wallet having this account, the restoring wallet should
    // verify that the birthday block exists within the main chain.
    //
    // [`Zewif`]: crate::Zewif
    birthday_block: Option<BlockHash>,

    // The ZIP 32 account ID used in derivation from an HD seed.
    zip32_account_id: Option<u32>,

    // The set of addresses that are associated with this account.
    addresses: Vec<Address>,

    // Subset of the global transaction history that involves this account.
    relevant_transactions: HashSet<TxId>,

    // The following are intended for storage of information that may not be
    // recoverable from the chain.
    sapling_sent_outputs: Vec<SaplingSentOutput>,
    orchard_sent_outputs: Vec<OrchardSentOutput>,
    attachments: Attachments,
}

#[rustfmt::skip]
impl std::fmt::Debug for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Account")
            .field("index", &self.index)
            .field("name", &self.name)
            .field("birthday_height", &self.birthday_height)
            .field("birthday_block", &self.birthday_block)
            .field("zip32_account_id", &NoQuotesDebugOption(&self.zip32_account_id))
            .field("addresses", &self.addresses)
            .field("relevant_transactions", &self.relevant_transactions)
            .field("sapling_sent_outputs", &self.sapling_sent_outputs)
            .field("orchard_sent_outputs", &self.orchard_sent_outputs)
            .field("attachments", &self.attachments)
            .finish()
    }
}

bc_envelope::impl_attachable!(Account);

impl Indexed for Account {
    fn index(&self) -> usize {
        self.index
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}

impl Account {
    pub fn new() -> Self {
        Self {
            index: 0,
            name: String::default(),
            birthday_height: None,
            birthday_block: None,
            zip32_account_id: None,
            addresses: Vec::new(),
            relevant_transactions: HashSet::new(),
            sapling_sent_outputs: Vec::new(),
            orchard_sent_outputs: Vec::new(),
            attachments: Attachments::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: impl Into<String>) {
        self.name = name.into();
    }

    pub fn birthday_height(&self) -> Option<BlockHeight> {
        self.birthday_height
    }

    pub fn set_birthday_height(&mut self, birthday_height: Option<BlockHeight>) {
        self.birthday_height = birthday_height;
    }

    pub fn birthday_block(&self) -> Option<BlockHash> {
        self.birthday_block
    }

    pub fn set_birthday_block(&mut self, birthday_block: Option<BlockHash>) {
        self.birthday_block = birthday_block;
    }

    pub fn zip32_account_id(&self) -> Option<u32> {
        self.zip32_account_id
    }

    pub fn set_zip32_account_id(&mut self, id: u32) {
        self.zip32_account_id = Some(id);
    }

    pub fn addresses(&self) -> &Vec<Address> {
        &self.addresses
    }

    pub fn addresses_len(&self) -> usize {
        self.addresses.len()
    }

    pub fn add_address(&mut self, mut address: Address) {
        address.set_index(self.addresses.len());
        self.addresses.push(address);
    }

    pub fn relevant_transactions(&self) -> &HashSet<TxId> {
        &self.relevant_transactions
    }

    pub fn relevant_transactions_len(&self) -> usize {
        self.relevant_transactions.len()
    }

    pub fn add_relevant_transaction(&mut self, txid: TxId) {
        self.relevant_transactions.insert(txid);
    }

    pub fn sapling_sent_outputs(&self) -> &Vec<SaplingSentOutput> {
        &self.sapling_sent_outputs
    }

    pub fn sapling_sent_outputs_len(&self) -> usize {
        self.sapling_sent_outputs.len()
    }

    pub fn add_sapling_sent_output(&mut self, mut output: SaplingSentOutput) {
        output.set_index(self.sapling_sent_outputs.len());
        self.sapling_sent_outputs.push(output);
    }

    pub fn orchard_sent_outputs(&self) -> &Vec<OrchardSentOutput> {
        &self.orchard_sent_outputs
    }

    pub fn orchard_sent_outputs_len(&self) -> usize {
        self.orchard_sent_outputs.len()
    }

    pub fn add_orchard_sent_output(&mut self, mut output: OrchardSentOutput) {
        output.set_index(self.orchard_sent_outputs.len());
        self.orchard_sent_outputs.push(output);
    }
}

impl Default for Account {
    fn default() -> Self {
        Self::new()
    }
}

#[rustfmt::skip]
impl From<Account> for Envelope {
    fn from(value: Account) -> Self {
        let mut e = Envelope::new(value.index)
            .add_type("Account")
            .add_assertion("name", value.name)
            .add_optional_assertion("birthday_height", value.birthday_height)
            .add_optional_assertion("birthday_block", value.birthday_block)
            .add_optional_assertion("zip32_account_id", value.zip32_account_id)
            .add_assertion("relevant_transactions", value.relevant_transactions.sort_by_cbor_encoding()); // Deterministic ordering

        e = value.addresses.iter().fold(e, |e, address| e.add_assertion("address", address.clone()));
        e = value.sapling_sent_outputs.iter().fold(e, |e, output| e.add_assertion("sapling_sent_output", output.clone()));
        e = value.orchard_sent_outputs.iter().fold(e, |e, output| e.add_assertion("orchard_sent_output", output.clone()));

        value.attachments.add_to_envelope(e)
    }
}

impl TryFrom<Envelope> for Account {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.check_type("Account")?;
        let index = envelope.extract_subject()?;
        let name = envelope.extract_object_for_predicate("name")?;
        let birthday_height = envelope.extract_optional_object_for_predicate("birthday_height")?;
        let birthday_block = envelope.extract_optional_object_for_predicate("birthday_block")?;
        let zip32_account_id = envelope.extract_optional_object_for_predicate("zip32_account_id")?;
        let relevant_transactions = envelope.extract_object_for_predicate("relevant_transactions")?;

        let addresses = envelope_indexed_objects_for_predicate(&envelope, "address")
            .map_err(|e| bc_envelope::Error::General(format!("addresses: {}", e)))?;
        let sapling_sent_outputs = envelope_indexed_objects_for_predicate(&envelope, "sapling_sent_output")
            .map_err(|e| bc_envelope::Error::General(format!("sapling_sent_outputs: {}", e)))?;
        let orchard_sent_outputs = envelope_indexed_objects_for_predicate(&envelope, "orchard_sent_output")
            .map_err(|e| bc_envelope::Error::General(format!("orchard_sent_outputs: {}", e)))?;

        let attachments = Attachments::try_from_envelope(&envelope)
            .map_err(|e| bc_envelope::Error::General(format!("attachments: {}", e)))?;

        Ok(Self {
            index,
            name,
            birthday_height,
            birthday_block,
            zip32_account_id,
            addresses,
            relevant_transactions,
            sapling_sent_outputs,
            orchard_sent_outputs,
            attachments,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use bc_envelope::Attachments;

    use crate::{BlockHash, BlockHeight, test_envelope_roundtrip};

    use super::Account;

    impl crate::RandomInstance for Account {
        fn random() -> Self {
            use crate::SetIndexes;

            Self {
                index: 0,
                name: String::random(),
                birthday_height: BlockHeight::opt_random(),
                birthday_block: BlockHash::opt_random(),
                zip32_account_id: u32::opt_random(),
                addresses: Vec::random().set_indexes(),
                relevant_transactions: HashSet::random(),
                sapling_sent_outputs: Vec::random().set_indexes(),
                orchard_sent_outputs: Vec::random().set_indexes(),
                attachments: Attachments::random(),
            }
        }
    }

    test_envelope_roundtrip!(Account);
}
