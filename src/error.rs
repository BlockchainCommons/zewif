use dcbor::prelude::CBORError;
use std::{
    array::TryFromSliceError, borrow::Cow, convert::Infallible,
    error::Error as StdError,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    // Catch-all string context that wraps any source error
    #[error("{message}")]
    Context {
        message: Cow<'static, str>,
        #[source]
        source: Box<dyn StdError + Send + Sync>,
    },

    // Validation errors
    #[error("Invalid network identifier: {0}")]
    InvalidNetwork(String),

    #[error("Amount underflow: {0}")]
    AmountUnderflow(u64),

    #[error("Amount overflow: {0}")]
    AmountOverflow(u64),

    #[error("Invalid SeedMaterial envelope")]
    InvalidSeedMaterial,

    #[error("Envelope is not a Zewif envelope")]
    NotZewifEnvelope,

    #[error(
        "Cannot compress a Zewif that has already been compressed or encrypted"
    )]
    AlreadyCompressedOrEncrypted,

    #[error("Cannot uncompress a Zewif that has not been compressed")]
    NotCompressed,

    #[error("Cannot encrypt a Zewif that has already been encrypted")]
    AlreadyEncrypted,

    #[error("Cannot decrypt a Zewif that has not been encrypted")]
    NotEncrypted,

    #[error("Invalid language value: {0}")]
    InvalidLanguage(String),

    #[error("Invalid MnemonicLanguage string: {0}")]
    InvalidMnemonicLanguage(String),

    #[error("Invalid TransparentSpendAuthority envelope")]
    InvalidTransparentSpendAuthority,

    #[error("Invalid ProtocolAddress type")]
    InvalidProtocolAddress,

    #[error("Hex parsing error: expected {expected} bytes, got {actual}")]
    HexLengthMismatch { expected: usize, actual: usize },

    #[error("Invalid hex string: {0}")]
    InvalidHexString(#[from] hex::FromHexError),

    #[error("Envelope error: {0}")]
    EnvelopeError(#[from] bc_envelope::Error),

    #[error("Slice conversion error: {0}")]
    TryFromSliceError(#[from] TryFromSliceError),

    #[error("CBOR error: {0}")]
    CBORError(#[from] CBORError),
}

impl From<Infallible> for Error {
    fn from(e: Infallible) -> Self {
        match e {}
    }
}

impl From<Error> for CBORError {
    fn from(e: Error) -> Self {
        match e {
            Error::CBORError(cbor_err) => cbor_err,
            other => CBORError::msg(other.to_string()),
        }
    }
}

impl From<Error> for bc_envelope::Error {
    fn from(e: Error) -> Self {
        match e {
            Error::EnvelopeError(envelope_err) => envelope_err,
            other => bc_envelope::Error::msg(other.to_string()),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
