//! Block definitions
use core::{fmt::Display, str::FromStr};

use crate::{
    transaction::Transaction,
    util::{as_hex_array, as_hex_buffer},
    Address, Error,
};
use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use serde::{Deserialize, Serialize};

/// Block ID (hash)
#[derive(
    Debug, Default, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
#[repr(transparent)]
pub struct BlockId(#[serde(with = "as_hex_array")] pub [u8; 32]);

impl FromStr for BlockId {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bytes = [0x00; 32];
        hex::decode_to_slice(s, &mut bytes).map_err(|_| Error::InvalidBlockId)?;
        Ok(Self(bytes))
    }
}

impl Display for BlockId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", hex::encode(self.0))
    }
}

impl From<alloy_primitives::BlockHash> for BlockId {
    fn from(value: alloy_primitives::BlockHash) -> Self {
        Self(value.0)
    }
}

impl From<BlockId> for alloy_primitives::BlockHash {
    fn from(value: BlockId) -> Self {
        Self(value.0)
    }
}

/// Block selector
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BlockBy {
    /// ById
    Id(BlockId),
    /// ByNumber
    Number(u64),
}

impl BlockBy {
    /// By id or number
    pub fn id_or_num(&self) -> String {
        match self {
            Self::Id(id) => id.to_string(),
            Self::Number(num) => num.to_string(),
        }
    }
}

/// Block raw data struct
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BlockRawData {
    /// Block number
    pub number: u64,
    /// Tx trie root
    #[serde(with = "as_hex_buffer", rename = "txTrieRoot")]
    pub tx_trie_root: Vec<u8>,
    /// Witness address
    pub witness_address: Address,
    /// Parent hash
    #[serde(with = "as_hex_buffer", rename = "parentHash")]
    pub parent_hash: Vec<u8>,
    /// Version
    pub version: u32,
    /// Block timestamp
    pub timestamp: u64,
}

/// Block header struct
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BlockHeader {
    /// raw data
    pub raw_data: BlockRawData,
    /// witness signature
    #[serde(with = "as_hex_buffer")]
    pub witness_signature: Vec<u8>,
}

impl BlockHeader {
    /// Get block number
    pub fn block_number(&self) -> u64 {
        self.raw_data.number
    }
}

/// Block struct
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Block {
    /// Block id
    #[serde(rename = "blockID")]
    pub block_id: BlockId,
    /// Block header
    pub block_header: BlockHeader,
    /// Transactions
    #[serde(default)]
    pub transactions: Vec<Transaction>,
}

impl Block {
    /// Get block number
    pub fn block_number(&self) -> u64 {
        self.block_header.block_number()
    }
}
