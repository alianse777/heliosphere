//! Signer error type

/// Error type
#[derive(Debug)]
pub enum SignerError {
    /// Failed to decode key
    KeyDecodeError,
    /// Failed to decode txid hex
    TxIdDecodeError,
    /// Invalid key bytes
    InvalidKey,
    /// Failed to sign message
    SignError(k256::ecdsa::Error),
}
