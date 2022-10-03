use cosmwasm_std::Coin;
use cosmwasm_schema::{cw_serde, QueryResponses};

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
        add_to_pool_amount: Coin,
        condition: Condition,
    },
    UpdateOsmoBaseReward{
        new_base_reward: Coin,
    },
    UpdateOwnerAddr{
        addr: String,
    },
}

#[cw_serde]
pub struct Condition{
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

