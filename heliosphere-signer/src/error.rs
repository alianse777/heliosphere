//! Signer error type

/// Error type
#[derive(Debug)]
#[cfg_attr(feature = "std", derive(thiserror::Error))]
pub enum SignerError {
    /// Failed to decode key
    #[cfg_attr(feature = "std", error("KeyDecodeError"))]
    KeyDecodeError,
    /// Failed to decode txid hex
    #[cfg_attr(feature = "std", error("TxIdDecodeError"))]
    TxIdDecodeError,
    /// Invalid key bytes
    #[cfg_attr(feature = "std", error("InvalidKey"))]
    InvalidKey,
}
