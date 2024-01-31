// schema.rs

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Uint128;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub admin: String,
    pub balances: std::collections::BTreeMap<String, u128>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub initial_balances: Vec<(String, u128)>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum HandleMsg {
    Deposit {},
    DistributeFunds { recipients: Vec<String>, amounts: Vec<u128> },
    Admin { new_admin: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    GetBalance {},
}

// Here you can define more schema types if needed for other parts of your contract
