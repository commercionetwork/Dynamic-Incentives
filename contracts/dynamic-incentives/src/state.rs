use cw_storage_plus::Item;

pub const DEBUG: Item<bool> = Item::new("debug");


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct DynamicIncentivesInfo {
    pub owner: String,
    pub base_amount: String,
}

