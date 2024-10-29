use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Transfer {
    pub from: Addr,
    pub to: String,
    pub amount: u128,
    pub status: TransferStatus,
    pub timestamp: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum TransferStatus {
    Pending,
    Completed,
    Failed,
}

// State storage
pub const CONFIG: Item<Config> = Item::new("config");
pub const TRANSFERS: Map<&str, Transfer> = Map::new("transfers");