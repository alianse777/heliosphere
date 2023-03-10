/// Error type
#[derive(Debug, Clone)]
pub enum Error {
    /// Invalid address bytes or address string
    InvalidAddress,
    /// Invalid transaction id
    InvalidTransactionId,
    /// Invalid block id
    InvalidBlockId,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}
