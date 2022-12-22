use cosmwasm_std::Coin;
use cosmwasm_schema::{cw_serde, QueryResponses};
use osmosis_std::shim::{Timestamp, Duration};
use osmosis_std::types::cosmos::base::v1beta1::Coin as CosmosCoin;
use osmosis_std::types::osmosis::lockup::QueryCondition;
use osmosis_std::types::osmosis::gamm::v1beta1::{PoolParams, PoolAsset};

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,  //owner has privileges to perform execute 
    pub osmo_base_reward: Coin, //amount of reward
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    AddToGauge{
        gauge_id: u64,
        owner: String,
        reward_amount: CosmosCoin,
    },
    CreateGauge{
        is_perpetual: bool,
        owner: String,
        distribute_to: Option<QueryCondition>,
        coins: Vec<CosmosCoin>,
        start_time:  Option<Timestamp>,
        num_epochs_paid_over: u64,
    },
    UpdateOsmoBaseReward{
        new_base_reward: Coin,
    },
    UpdateOwnerAddr{
        addr: String,
    },
    CreateDenom {
        subdenom: String,
        initial_mint: Option<String>,
        initial_pool: Option<InitPoolCfg>,
    },
    LockTokens {
        owner: String,
        //duration: Option<Duration>,
        coins: Vec<CosmosCoin>,
    },
    CreatePool{
        sender: String,
        pool_params: Option<PoolParams>,
        pool_assets: Vec<PoolAsset>,
        future_pool_governor: String,
    },
}
/*
#[cw_serde]
pub struct Condition{
}*/
#[cw_serde]
pub struct InitPoolCfg {
    pub swap_fee: String,
    pub exit_fee: String,
    pub pairing_denom: String,
    pub pool_assets: PoolAssests,
}

#[cw_serde]
pub struct PoolAssests {
    pub new_token_amount: String,
    pub new_token_weight: String,
    pub pairing_token_amount: String,
    pub pairing_token_weight: String,
}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(InfoResponse)]
    Info {},

    #[returns(GetOwnerResponse)]
    GetOwner {},
    
    #[returns(GetOsmoBaseRewardResponse)]
    GetOsmoBaseReward {},
}

#[cw_serde]
pub struct InfoResponse {
}

#[cw_serde]
pub struct GetOwnerResponse {
    pub owner: String,
}

#[cw_serde]
pub struct GetOsmoBaseRewardResponse {
    pub osmo_base_reward: Coin,
}

