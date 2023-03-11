use heliosphere_core::transaction::{Transaction, TransactionId};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone)]
pub struct BroadcastTxResponse {
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub message: String,
    pub txid: TransactionId,
}

#[derive(Deserialize, Debug, Clone)]
pub struct QueryContractResponse {
    #[serde(default)]
    pub code: Option<String>,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub constant_result: Vec<String>,
    #[serde(default)]
    pub energy_used: u64,
}

impl QueryContractResponse {
    pub fn constant_result(&self, index: usize) -> Result<Vec<u8>, crate::Error> {
        let res = self
            .constant_result
            .get(index)
            .ok_or(crate::Error::InvalidIndex)?;
        hex::decode(res).map_err(|e| crate::Error::UnknownResponse(e.to_string()))
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TriggerContractResponse {
    pub transaction: Transaction,
}

/// Resource type: Energy or Bandwidth
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "UPPERCASE")]
pub enum ResourceType {
    /// Bandwidth resource
    Bandwidth,
    /// Energy resource
    Energy,
}

/// Account resources
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountResources {
    /// Free bandwidth used
    #[serde(rename = "freeNetUsed", default)]
    pub free_net_used: u64,
    /// Total free bandwidth
    #[serde(rename = "freeNetLimit", default)]
    pub free_net_limit: u64,
    /// Used amount of bandwidth obtained by staking
    #[serde(default)]
    pub net_used: u64,
    /// Total bandwidth obtained by staking
    #[serde(default)]
    pub net_limit: u64,
    /// Total bandwidth that can be obtained by staking
    #[serde(default)]
    pub total_net_limit: u64,
    /// Total TRX staked for bandwidth
    #[serde(default)]
    pub total_net_weight: u64,
    /// TRON Power(vote)
    #[serde(rename = "tronPowerLimit", default)]
    pub tron_power_limit: u64,
    /// Energy used
    #[serde(default)]
    pub energy_used: u64,
    /// Total energy obtained by staking
    #[serde(default)]
    pub energy_limit: u64,
    /// Total energy that can be obtained by staking
    #[serde(default)]
    pub total_energy_limit: u64,
    /// Total TRX staked for energy
    #[serde(default)]
    pub total_energy_weight: u64,
}

/// Transaction receipt
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionRet {
    #[serde(rename = "contractRet")]
    pub contract_ret: String,
}

/// Transaction info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionInfo {
    #[serde(flatten)]
    pub transaction: Transaction,
    pub ret: Vec<TransactionRet>,
}

/// Chain parameter (key, value)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub(crate) struct ChainParameter {
    pub key: String,
    #[serde(default)]
    pub value: Option<i64>,
}

/// Chain parameters as returned by GetChainParameters
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub(crate) struct ChainParametersResponse {
    #[serde(rename = "chainParameter")]
    pub chain_parameter: Vec<ChainParameter>,
}

/// Account info (as returned by /wallet/getaccount)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub(crate) struct AccountBalanceResponse {
    #[serde(default)]
    pub balance: Option<u64>,
}
