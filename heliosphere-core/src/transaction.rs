//! Transaction definitions
#![allow(missing_docs)]
use crate::util::{as_hex_array, as_hex_buffer};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use serde::{Deserialize, Serialize};

/// Transaction id wrapper, serializable as hex string
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct TransactionId(#[serde(with = "as_hex_array")] pub [u8; 32]);

impl core::fmt::Display for TransactionId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl core::fmt::Debug for TransactionId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl core::str::FromStr for TransactionId {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            hex::decode(s)
                .map_err(|_| crate::Error::InvalidTransactionId)?
                .try_into()
                .map_err(|_| crate::Error::InvalidTransactionId)?,
        ))
    }
}

impl From<alloy_primitives::TxHash> for TransactionId {
    fn from(value: alloy_primitives::TxHash) -> Self {
        Self(value.0)
    }
}

impl From<TransactionId> for alloy_primitives::TxHash {
    fn from(value: TransactionId) -> Self {
        Self(value.0)
    }
}

/// Contract call with json payload
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Contract {
    /// Param
    pub parameter: serde_json::Value,
    /// Type
    pub r#type: String,
}

/// See tron docs for field description
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RawTxData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    pub contract: Vec<Contract>,
    pub expiration: u64,
    pub timestamp: u64,
    #[serde(default)]
    pub fee_limit: u64,
    #[serde(with = "as_hex_buffer")]
    pub ref_block_bytes: Vec<u8>,
    #[serde(with = "as_hex_buffer")]
    pub ref_block_hash: Vec<u8>,
}

/// See tron docs for field description
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Transaction {
    pub raw_data: RawTxData,
    #[serde(with = "as_hex_buffer", rename = "raw_data_hex")]
    pub raw_data_bytes: Vec<u8>,
    #[serde(rename = "txID")]
    pub tx_id: TransactionId,
    #[serde(default)]
    pub signature: Vec<String>,
}

impl Transaction {
    /// Get tx id as hex string
    pub fn tx_id_hex(&self) -> String {
        self.tx_id.to_string()
    }

    /// Get raw data as hex string
    pub fn raw_data_hex(&self) -> String {
        hex::encode(&self.raw_data_bytes)
    }
}
