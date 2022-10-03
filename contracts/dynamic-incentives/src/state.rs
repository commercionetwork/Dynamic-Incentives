// use `cw_storage_plus` to create ORM-like interface to storage
// see: https://crates.io/crates/cw-storage-plus

use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;

// OWNER stores the contract owner configured at instantiation time.
pub const OWNER: Item<Addr> = Item::new("owner");
// OSMO_BASE_REWARD stores the base reward for incentives
pub const OSMO_BASE_REWARD: Item<Coin> = Item::new("osmo_base_reward");
