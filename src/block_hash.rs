use crate::error::{Error, Result};
use bc_envelope::prelude::*;
use std::{
    fmt,
    io::{self, Read, Write},
};

/// A transaction identifier (BlockHash) represented as a 32-byte hash.
///
/// `BlockHash` is a specialized wrapper around a 32-byte array representing a block's
/// unique identifier in the Zcash blockchain.
///
/// # Zcash Concept Relation
/// In Zcash (and Bitcoin-derived cryptocurrencies), transaction IDs are critical identifiers
/// used to reference transactions throughout the protocol:
/// - In transaction inputs to reference previous outputs being spent
/// - In block data structures to identify included transactions
/// - In client APIs and explorers to look up transaction details
///
/// Block hashes are displayed in reverse byte order by convention (to match Bitcoin's historical
/// display format), while stored internally in little-endian order.
///
/// # Data Preservation
/// The `BlockHash` type preserves the exact 32-byte transaction identifier as found in wallet
/// data files, ensuring that transaction references maintain their cryptographic integrity
/// during wallet migrations.
///
/// # Examples
/// ```
/// # use zewif::BlockHash;
/// // Create a BlockHash from a byte array
/// let tx_bytes = [0u8; 32];
/// let txid = BlockHash::from_bytes(tx_bytes);
///
/// // Display the BlockHash in the conventional reversed format used by explorers
/// // Note: this would display as a string of 64 hex characters (zeros in this example)
/// println!("Block Hash: {}", txid);
/// ```
#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct BlockHash([u8; 32]);

impl fmt::Debug for BlockHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BlockHash({})", self)
    }
}

impl fmt::Display for BlockHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The (byte-flipped) hex string is more useful than the raw bytes, because we can
        // look that up in RPC methods and block explorers.
        let mut data = self.0;
        data.reverse();
        f.write_str(&hex::encode(data))
    }
}

impl AsRef<[u8; 32]> for BlockHash {
    fn as_ref(&self) -> &[u8; 32] {
        &self.0
    }
}

impl From<BlockHash> for [u8; 32] {
    fn from(value: BlockHash) -> Self {
        value.0
    }
}

impl BlockHash {
    /// Creates a new `BlockHash` from a 32-byte array.
    ///
    /// This is the primary constructor for `BlockHash` when you have the raw transaction
    /// hash available.
    ///
    /// # Examples
    /// ```
    /// # use zewif::BlockHash;
    /// // Usually this would be a real transaction hash
    /// let bytes = [0u8; 32];
    /// let txid = BlockHash::from_bytes(bytes);
    /// ```
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        BlockHash(bytes)
    }

    /// Parses a `BlockHash` from a canonically-encoded (byte-reversed) hexadecimal string.
    ///
    /// # Examples
    /// ```
    /// # use zewif::BlockHash;
    ///
    /// let hex = "000000000019d6689c085ae165831e934ff763ae46a2a6c172b3f1b60a8ce26f";
    /// let block_hash = BlockHash::from_hex(hex).unwrap();
    /// assert_eq!(block_hash.as_ref()[0], 0x6f);
    /// assert_eq!(format!("{}", block_hash), hex);
    /// ```
    pub fn from_hex(hex: &str) -> Result<Self> {
        let mut data = hex::decode(hex)?;
        data.reverse();

        Ok(Self(<[u8; 32]>::try_from(&data[..]).map_err(|_| {
            Error::HexLengthMismatch {
                expected: 32,
                actual: data.len(),
            }
        })?))
    }

    /// Reads a `BlockHash` from any source implementing the `Read` trait.
    ///
    /// This method is useful when reading transaction IDs directly from files
    /// or other byte streams.
    ///
    /// # Errors
    /// Returns an IO error if reading fails or if there aren't enough bytes available.
    ///
    /// # Examples
    /// ```no_run
    /// # use std::io::Cursor;
    /// # use zewif::BlockHash;
    /// #
    /// # fn example() -> std::io::Result<()> {
    /// // Create a cursor with 32 bytes
    /// let data = vec![0u8; 32];
    /// let mut cursor = Cursor::new(data);
    ///
    /// // Read a BlockHash from the cursor
    /// let txid = BlockHash::read(&mut cursor)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read<R: Read>(mut reader: R) -> io::Result<Self> {
        let mut hash = [0u8; 32];
        reader.read_exact(&mut hash)?;
        Ok(BlockHash::from_bytes(hash))
    }

    /// Writes a `BlockHash` to any destination implementing the `Write` trait.
    ///
    /// This method is useful when serializing transaction IDs to files or
    /// other byte streams.
    ///
    /// # Errors
    /// Returns an IO error if writing fails.
    ///
    /// # Examples
    /// ```no_run
    /// # use std::io::Cursor;
    /// # use zewif::BlockHash;
    /// #
    /// # fn example() -> std::io::Result<()> {
    /// let txid = BlockHash::from_bytes([0u8; 32]);
    /// let mut buffer = Vec::new();
    ///
    /// // Write the BlockHash to the buffer
    /// txid.write(&mut buffer)?;
    ///
    /// // The buffer now contains the 32-byte transaction ID
    /// assert_eq!(buffer.len(), 32);
    /// # Ok(())
    /// # }
    /// ```
    pub fn write<W: Write>(&self, mut writer: W) -> io::Result<()> {
        writer.write_all(&self.0)?;
        Ok(())
    }
}

impl From<BlockHash> for CBOR {
    fn from(value: BlockHash) -> Self {
        CBOR::to_byte_string(value.0)
    }
}

impl From<&BlockHash> for CBOR {
    fn from(value: &BlockHash) -> Self {
        CBOR::to_byte_string(value.0)
    }
}

impl TryFrom<CBOR> for BlockHash {
    type Error = dcbor::Error;

    fn try_from(cbor: CBOR) -> dcbor::Result<Self> {
        let bytes = cbor.try_into_byte_string()?;
        if bytes.len() != 32 {
            return Err(format!(
                "Invalid BlockHash length: expected 32 bytes, got {}",
                bytes.len()
            )
            .into());
        }
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&bytes);
        Ok(BlockHash::from_bytes(hash))
    }
}

impl From<BlockHash> for Envelope {
    fn from(value: BlockHash) -> Self {
        Envelope::new(CBOR::from(value))
    }
}

impl TryFrom<Envelope> for BlockHash {
    type Error = bc_envelope::Error;

    fn try_from(envelope: Envelope) -> bc_envelope::Result<Self> {
        envelope.extract_subject()
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_cbor_roundtrip, test_envelope_roundtrip};

    use super::BlockHash;

    impl crate::RandomInstance for BlockHash {
        fn random() -> Self {
            let mut rng = bc_rand::thread_rng();
            Self(bc_rand::rng_random_array(&mut rng))
        }
    }

    test_cbor_roundtrip!(BlockHash);
    test_envelope_roundtrip!(BlockHash);
}
