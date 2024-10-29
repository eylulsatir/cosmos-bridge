use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    InitiateTransfer {
        to: String,
        amount: u128,
    },
    CompleteTransfer {
        transfer_id: String,
    },
    ToggleBridge {
        enabled: bool,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetConfig {},
    GetTransfer {
        transfer_id: String,
    },
    ListTransfers {
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

// Query responses
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: String,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TransfersResponse {
    pub transfers: Vec<TransferInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TransferInfo {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: u128,
    pub status: String,
    pub timestamp: u64,
}