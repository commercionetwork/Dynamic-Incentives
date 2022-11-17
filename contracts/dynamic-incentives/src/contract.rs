#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult, Coin};
use cw2::set_contract_version;
use osmosis_std::types::osmosis::incentives::{MsgAddToGauge};
use osmosis_std::types::cosmos::base::v1beta1::Coin as CosmosCoin;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, GetOwnerResponse, InfoResponse, GetOsmoBaseRewardResponse};
use crate::state::{OWNER, OSMO_BASE_REWARD};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:dynamic-incentives";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner_address = deps.api.addr_validate(&msg.owner)?;

    OWNER.save(deps.storage, &owner_address)?;
    OSMO_BASE_REWARD.save(deps.storage, &msg.osmo_base_reward)?;

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
            owner,
            reward_amount,
        } => add_to_gauge(deps, info, env, gauge_id, owner, reward_amount),
        ExecuteMsg::UpdateOsmoBaseReward { new_base_reward } => update_base_reward(deps, info, new_base_reward),
        ExecuteMsg::UpdateOwnerAddr { addr } => update_owner_addr(deps, info, addr),
    }
}

pub fn add_to_gauge(
    _deps: DepsMut,
    _info: MessageInfo,
    _env: Env,
    gauge_id: u64,
    owner: String,
    reward_amount: Vec<CosmosCoin>,
) -> Result<Response, ContractError> {
    
    let msg_add_to_gauge : CosmosMsg = MsgAddToGauge {
            owner,
            gauge_id,
            rewards: reward_amount,
    }.into();

    Ok(Response::new()
        .add_message(msg_add_to_gauge)
        .add_attribute("method", "add_to_gauge")    
    )
}

pub fn update_base_reward(
    deps: DepsMut,
    _info: MessageInfo,
    new_base_reward: Coin,
) -> Result<Response, ContractError> {

    OSMO_BASE_REWARD.update(deps.storage, |mut base_reward| -> Result<_, ContractError> {
        if base_reward != new_base_reward {
            base_reward = new_base_reward;
        }
        Ok(base_reward)
    })?;

    Ok(Response::new())
}

pub fn update_owner_addr(
    deps: DepsMut,
    _info: MessageInfo,
    addr: String,
) -> Result<Response, ContractError> {
    
    OWNER.update(deps.storage, |mut owner| -> Result<_, ContractError> {
        let new_owner = deps.api.addr_validate(&addr)?;
        if owner != new_owner {
            owner = deps.api.addr_validate(&addr)?;
        }
        Ok(owner)
    })?;

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

pub fn query_info(_deps: Deps) -> StdResult<InfoResponse> {
    Ok(InfoResponse{})
}

pub fn query_owner(deps: Deps) -> StdResult<GetOwnerResponse> {
    let owner = OWNER.load(deps.storage)?;
    Ok(GetOwnerResponse {
        owner: owner.into_string(),
    })
}

pub fn query_osmo_base_reward(deps: Deps) -> StdResult<GetOsmoBaseRewardResponse> {
    let osmo_base_reward = OSMO_BASE_REWARD.load(deps.storage)?;
    Ok(GetOsmoBaseRewardResponse {
        osmo_base_reward: osmo_base_reward,
    })
}

/*
/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, _msg: Reply) -> Result<Response, ContractError> {
    // With `Response` type, it is still possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages

    todo!()
}*/
