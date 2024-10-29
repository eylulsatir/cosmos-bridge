use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Transfer already exists")]
    TransferExists {},

    #[error("Bridge is disabled")]
    BridgeDisabled {},

    #[error("Invalid transfer amount")]
    InvalidAmount {},
}