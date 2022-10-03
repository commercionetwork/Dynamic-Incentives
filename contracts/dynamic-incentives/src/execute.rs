use cosmwasm_std::{coin, has_coins, Coin, DepsMut, Env, MessageInfo, Response, SubMsg};
use osmosis_std::types::osmosis::gamm::v1beta1::SwapAmountInRoute;

use crate::contract::SWAP_REPLY_ID;
use crate::helpers::{
    calculate_min_output_from_twap, generate_swap_msg, validate_is_contract_owner,
    validate_pool_route,
};
use crate::msg::SwapType;
use crate::state::{SwapMsgReplyState, ROUTING_TABLE, SWAP_REPLY_STATES};
use crate::ContractError;

// increment_gauge sets incentives
pub fn increment_gauge(
    deps: DepsMut,
    info: MessageInfo,
    gauge_id: String,
    condition: String,
    coins: String,
) -> Result<Response, ContractError> {
 
    // Implementation


    Ok(Response::new().add_attribute("action", "increment_gauge"))
}



// set_base sets base
pub fn set_base(
    deps: DepsMut,
    info: MessageInfo,
    base_amount: String,,
) -> Result<Response, ContractError> {
 
    // Implementation


    Ok(Response::new().add_attribute("action", "set_base"))
}


// set_owner sets owner
pub fn set_owner(
    deps: DepsMut,
    info: MessageInfo,
    owner: String,
) -> Result<Response, ContractError> {
 
    // Implementation


    Ok(Response::new().add_attribute("action", "set_owner"))
}