use cosmwasm_std::{
    to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    entry_point, Order, StdError,
};
use cw_storage_plus::Bound;

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, QueryMsg, ConfigResponse, TransfersResponse,
    TransferInfo,
};
use crate::state::{Config, Transfer, TransferStatus, CONFIG, TRANSFERS};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        owner: info.sender.clone(),
        enabled: msg.enabled,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("enabled", msg.enabled.to_string()))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::InitiateTransfer { to, amount } => {
            execute_initiate_transfer(deps, env, info, to, amount)
        }
        ExecuteMsg::CompleteTransfer { transfer_id } => {
            execute_complete_transfer(deps, env, info, transfer_id)
        }
        ExecuteMsg::ToggleBridge { enabled } => {
            execute_toggle_bridge(deps, info, enabled)
        }
    }
}

pub fn execute_initiate_transfer(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    to: String,
    amount: u128,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if !config.enabled {
        return Err(ContractError::BridgeDisabled {});
    }

    if amount == 0 {
        return Err(ContractError::InvalidAmount {});
    }

    let transfer_id = format!("transfer_{}", env.block.height);
    let transfer = Transfer {
        from: info.sender.clone(),
        to: to.clone(),
        amount,
        status: TransferStatus::Pending,
        timestamp: env.block.time.seconds(),
    };

    TRANSFERS.save(deps.storage, &transfer_id, &transfer)?;

    Ok(Response::new()
        .add_attribute("action", "initiate_transfer")
        .add_attribute("transfer_id", transfer_id)
        .add_attribute("from", info.sender)
        .add_attribute("to", to)
        .add_attribute("amount", amount.to_string()))
}

pub fn execute_complete_transfer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    transfer_id: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    TRANSFERS.update(deps.storage, &transfer_id, |transfer_option| -> StdResult<_> {
        let mut transfer = transfer_option.ok_or_else(|| {
            StdError::generic_err("Transfer not found")
        })?;
        transfer.status = TransferStatus::Completed;
        Ok(transfer)
    })?;

    Ok(Response::new()
        .add_attribute("action", "complete_transfer")
        .add_attribute("transfer_id", transfer_id))
}

pub fn execute_toggle_bridge(
    deps: DepsMut,
    info: MessageInfo,
    enabled: bool,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    if info.sender != config.owner {
        return Err(ContractError::Unauthorized {});
    }

    config.enabled = enabled;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "toggle_bridge")
        .add_attribute("enabled", enabled.to_string()))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_json_binary(&query_config(deps)?),
        QueryMsg::GetTransfer { transfer_id } => {
            to_json_binary(&query_transfer(deps, transfer_id)?)
        }
        QueryMsg::ListTransfers { start_after, limit } => {
            to_json_binary(&query_list_transfers(deps, start_after, limit)?)
        }
    }
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: config.owner.to_string(),
        enabled: config.enabled,
    })
}

fn query_transfer(deps: Deps, transfer_id: String) -> StdResult<TransferInfo> {
    let transfer = TRANSFERS.load(deps.storage, &transfer_id)?;
    Ok(TransferInfo {
        id: transfer_id,
        from: transfer.from.to_string(),
        to: transfer.to,
        amount: transfer.amount,
        status: format!("{:?}", transfer.status),
        timestamp: transfer.timestamp,
    })
}

fn query_list_transfers(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<TransfersResponse> {
    let limit = limit.unwrap_or(30) as usize;
    let start = start_after.as_deref().map(Bound::exclusive);

    let transfers: StdResult<Vec<TransferInfo>> = TRANSFERS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|item| {
            let (key, transfer) = item?;
            Ok(TransferInfo {
                id: String::from_utf8(key.into())?,
                from: transfer.from.to_string(),
                to: transfer.to,
                amount: transfer.amount,
                status: format!("{:?}", transfer.status),
                timestamp: transfer.timestamp,
            })
        })
        .collect();

    Ok(TransfersResponse {
        transfers: transfers?,
    })
}
  