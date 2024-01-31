use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State};
use cosmwasm_std::{entry_point, to_binary, Binary, CosmosMsg, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdResult, WasmMsg};
use serde::{Deserialize, Serialize};

mod tests;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {
    pub initial_balances: Vec<BalanceEntry>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryMsg;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryAnswer {
    GetBalance { result: Vec<(String, u128)> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum HandleAnswer {
    Deposit {},
    DistributeFunds {},
    Admin {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BalanceEntry {
    pub address: String,
    pub balance: u128,
}

const ADMIN_KEY: &[u8] = b"admin_key";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: InitMsg) -> StdResult<Response> {
    Contract::instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: HandleMsg) -> StdResult<Response> {
    Contract::execute(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    Contract::query(deps, env, msg)
}

#[cfg(test)]
mod tests {
    // your tests here
}
