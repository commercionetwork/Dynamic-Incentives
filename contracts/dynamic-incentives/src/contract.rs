#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult, Coin};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, Condition, GetOwnerResponse, InfoResponse, GetOsmoBaseRewardResponse};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:dynamic-incentives";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // With `Response` type, it is possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddToGauge { 
            gauge_id, 
            add_to_pool_amount, 
            condition 
        } => add_to_gauge(deps, info, env, gauge_id, add_to_pool_amount, condition),
        ExecuteMsg::UpdateOsmoBaseReward { new_base_reward } => update_base_reward(deps, info, new_base_reward),
        ExecuteMsg::UpdateOwnerAddr { addr } => update_owner_addr(deps, info, addr),
    }
}

pub fn add_to_gauge(
    deps: DepsMut,
    info: MessageInfo,
    env: Env,
    gauge_id: u64,
    add_to_pool_amount: Coin,
    condition: Condition,
) -> Result<Response, ContractError> {

    Ok(Response::new())
}

pub fn update_base_reward(
    deps: DepsMut,
    info: MessageInfo,
    new_base_reward: Coin,
) -> Result<Response, ContractError> {

    Ok(Response::new())
}

pub fn update_owner_addr(
    deps: DepsMut,
    info: MessageInfo,
    addr: String,
) -> Result<Response, ContractError> {

    Ok(Response::new())
}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Info {} => to_binary(&query_info(deps)?),
        QueryMsg::GetOwner {} => to_binary(&query_owner(deps)?),
        QueryMsg::GetOsmoBaseReward {} => to_binary(&query_osmo_base_reward(deps)?),
    }
}

pub fn query_info(deps: Deps) -> StdResult<InfoResponse> {
    Ok(InfoResponse{})
}

pub fn query_owner(deps: Deps) -> StdResult<GetOwnerResponse> {
    Ok(GetOwnerResponse { owner: String::new()})
}

pub fn query_osmo_base_reward(deps: Deps) -> StdResult<GetOsmoBaseRewardResponse> {
    Ok(GetOsmoBaseRewardResponse{})
}

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> Result<Response, ContractError> {
    // With `Response` type, it is still possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages

    todo!()
}
