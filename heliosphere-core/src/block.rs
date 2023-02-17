//! Block definitions
use crate::{transaction::Transaction, util::as_hex_buffer, Address};
use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
    vec::Vec,
};
use serde::{Deserialize, Serialize};

/// Block identifier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BlockBy {
    /// ById
    Id(String),
    /// ByNumber
    Number(u64),
}

impl BlockBy {
    /// By id or number
    pub fn id_or_num(&self) -> String {
        match self {
            Self::Id(id) => id.to_owned(),
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
    pub block_id: String,
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
