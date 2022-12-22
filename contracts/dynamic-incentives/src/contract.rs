#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, Reply, Response,
    StdResult, SubMsg, SubMsgResponse, SubMsgResult, Uint128,
};
use cw2::set_contract_version;
use osmosis_std::shim::{Timestamp, Duration};
use osmosis_std::types::cosmos::base::v1beta1::Coin as CosmosCoin;
use osmosis_std::types::osmosis::gamm::poolmodels::balancer::v1beta1::{
    MsgCreateBalancerPool, MsgCreateBalancerPoolResponse,
};
use osmosis_std::types::osmosis::gamm::v1beta1::{PoolAsset, PoolParams};
use osmosis_std::types::osmosis::incentives::{MsgAddToGauge, MsgCreateGauge};
use osmosis_std::types::osmosis::lockup::{QueryCondition, MsgLockTokens};
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgCreateDenom, MsgMint};
use std::str::FromStr;

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, GetOsmoBaseRewardResponse, GetOwnerResponse, InfoResponse, InitPoolCfg,
    InstantiateMsg, QueryMsg,
};
use crate::state::{OSMO_BASE_REWARD, OWNER};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:dynamic-incentives";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const CREATE_POOL_REPLY_ID: u64 = 1;
//const ADD_TO_GAUGE_REPLY_ID: u64 = 1u64;

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
        ExecuteMsg::CreateGauge { is_perpetual, owner, distribute_to, coins, start_time, num_epochs_paid_over } => {
            create_gauge(deps, info, env, is_perpetual, owner, distribute_to, coins, start_time, num_epochs_paid_over)
        }
        ExecuteMsg::UpdateOsmoBaseReward { new_base_reward } => {
            update_base_reward(deps, info, new_base_reward)
        }
        ExecuteMsg::UpdateOwnerAddr { addr } => update_owner_addr(deps, info, addr),
        ExecuteMsg::CreateDenom {
            subdenom,
            initial_mint,
            initial_pool,
        } => try_create_denom(env, subdenom, initial_mint, initial_pool),
        ExecuteMsg::LockTokens {
            owner,
            //duration,
            coins,
        } => try_lock_tokens(env, owner, /*duration,*/ coins),
        ExecuteMsg::CreatePool {
            sender,
            pool_params,
            pool_assets,
            future_pool_governor,
        } => try_create_pool(env, sender, pool_params, pool_assets, future_pool_governor),
    }
}

pub fn add_to_gauge(
    _deps: DepsMut,
    info: MessageInfo,
    env: Env,
    gauge_id: u64,
    owner: String,
    reward_amount: CosmosCoin,
) -> Result<Response, ContractError> {
    let amount = Uint128::from_str(&reward_amount.amount.clone())?;
    // validate funds
    validate_input_amount(
        &info.funds, 
        amount, 
        reward_amount.denom.clone())?;

    // transfer funds to the contract address
    let transfer_bank_msg: CosmosMsg = cosmwasm_std::BankMsg::Send {
        to_address: env.contract.address.to_string(),
        amount: vec![Coin {
            denom: reward_amount.denom.clone(),
            amount: amount,
        }],
    }.into();
    
    let msg_add_to_gauge: CosmosMsg = MsgAddToGauge {
        owner,
        gauge_id,
        rewards: vec![reward_amount],
    }
    .into();

    Ok(Response::new()
        .add_messages(vec![
            transfer_bank_msg,
            msg_add_to_gauge,
        ])
        .add_attribute("method", "add_to_gauge"))
}

fn validate_input_amount(
    actual_funds: &[Coin],
    given_amount: Uint128,
    given_denom: String,
) -> Result<(), ContractError> {
    let actual = get_amount_for_denom(actual_funds, &given_denom);
    
    if actual.amount != given_amount {
        return Err(ContractError::InsufficientFunds {});
    }
    if &actual.denom != &given_denom {
        return Err(ContractError::IncorrectDenom {
            provided: actual.denom,
            required: given_denom,
        });
    };
    Ok(())
}

fn get_amount_for_denom(coins: &[Coin], denom: &str) -> Coin {
    let amount: Uint128 = coins
        .iter()
        .filter(|c| c.denom == denom)
        .map(|c| c.amount)
        .sum();
    Coin {
        amount,
        denom: denom.to_string(),
    }
}

pub fn create_gauge(
    _deps: DepsMut,
    _info: MessageInfo,
    _env: Env,
    is_perpetual: bool, 
    owner: String, 
    distribute_to: Option<QueryCondition>, 
    coins: Vec<CosmosCoin>, 
    start_time: Option<Timestamp>, 
    num_epochs_paid_over: u64,
) -> Result<Response, ContractError> {
    
    let msg_create_gauge: CosmosMsg = MsgCreateGauge{
        is_perpetual,
        owner,
        distribute_to,
        coins,
        start_time,
        num_epochs_paid_over,
    }.into();

    Ok(Response::new()
        .add_message(msg_create_gauge)
        .add_attribute("method", "create_gauge"))
}

pub fn update_base_reward(
    deps: DepsMut,
    _info: MessageInfo,
    new_base_reward: Coin,
) -> Result<Response, ContractError> {
    OSMO_BASE_REWARD.update(
        deps.storage,
        |mut base_reward| -> Result<_, ContractError> {
            if base_reward != new_base_reward {
                base_reward = new_base_reward;
            }
            Ok(base_reward)
        },
    )?;

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

pub fn try_create_denom(
    env: Env,
    subdenom: String,
    initial_mint: Option<String>,
    initial_pool: Option<InitPoolCfg>,
) -> Result<Response, ContractError> {
    let contract_addr = env.contract.address.to_string();

    let msg_create_denom: CosmosMsg = MsgCreateDenom {
        sender: contract_addr.clone(),
        subdenom: subdenom.clone(),
    }
    .into();

    let mut msgs = vec![SubMsg::new(msg_create_denom)];

    if let Some(initial_mint) = initial_mint {
        let msg_mint: CosmosMsg = MsgMint {
            sender: contract_addr.clone(),
            amount: Some(CosmosCoin {
                denom: format!("factory/{contract_addr}/{subdenom}"),
                amount: initial_mint,
            }),
        }
        .into();

        msgs.push(SubMsg::new(msg_mint));

        if let Some(InitPoolCfg {
            swap_fee,
            exit_fee,
            pairing_denom,
            pool_assets,
        }) = initial_pool
        {
            let msg_create_pool: CosmosMsg = MsgCreateBalancerPool {
                sender: contract_addr.clone(),
                pool_params: PoolParams {
                    swap_fee,
                    exit_fee,
                    smooth_weight_change_params: None,
                }
                .into(),
                pool_assets: vec![
                    PoolAsset {
                        token: CosmosCoin {
                            denom: format!("factory/{contract_addr}/{subdenom}"),
                            amount: pool_assets.new_token_amount,
                        }
                        .into(),
                        weight: pool_assets.new_token_weight,
                    },
                    PoolAsset {
                        token: CosmosCoin {
                            denom: pairing_denom,
                            amount: pool_assets.pairing_token_amount,
                        }
                        .into(),
                        weight: pool_assets.pairing_token_weight,
                    },
                ],
                future_pool_governor: contract_addr,
            }
            .into();

            msgs.push(SubMsg::reply_on_success(
                msg_create_pool,
                CREATE_POOL_REPLY_ID,
            ));
        }
    };

    Ok(Response::new()
        .add_submessages(msgs)
        .add_attribute("method", "try_create_denom"))
}

pub fn try_lock_tokens(
    _env: Env,
    owner: String,
    //duration: Option<Duration>,
    coins: Vec<CosmosCoin>,
) -> Result<Response, ContractError> {    
    let duration = Some(Duration{seconds:3600, nanos:1000});
    let msg_lock_tokens: CosmosMsg = MsgLockTokens {
        owner,
        duration,
        coins,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_lock_tokens")
        .add_message(msg_lock_tokens))
}

pub fn try_create_pool(
    _env: Env,
    sender: String,
    pool_params: Option<PoolParams>,
    pool_assets: Vec<PoolAsset>,
    future_pool_governor: String
) -> Result<Response, ContractError> {    
    let msg_create_pool: CosmosMsg = MsgCreateBalancerPool {
        sender,
        pool_params,
        pool_assets,
        future_pool_governor,
    }
    .into();

    Ok(Response::new()
        .add_attribute("method", "try_create_pool")
        .add_message(msg_create_pool))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(_deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    if msg.id == CREATE_POOL_REPLY_ID {
        if let SubMsgResult::Ok(SubMsgResponse { data: Some(b), .. }) = msg.result {
            // This is only for response deserialization demonstration purpose.
            // `pool_id` can actually be retrieved from `pool_created` event.
            let res: MsgCreateBalancerPoolResponse = b.try_into().map_err(ContractError::Std)?;
            return Ok(Response::new().add_attribute("pool_id", format!("{}", res.pool_id)));
        }
    };

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
