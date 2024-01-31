use cosmwasm_std::{Deps, Env, QueryRequest, StdResult, WasmQuery, BankQuery, to_binary};
use crate::contract;  // Ensure this line is present
use crate::msg::QueryMsg;
use crate::state::config_read;

pub fn handle_query(deps: Deps, env: Env, request: QueryRequest) -> StdResult<Vec<u8>> {
    match request {
        QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }) => {
            // Dispatch the query to the contract's query method
            contract::query(deps, env, msg)
        }
        QueryRequest::Bank(BankQuery::AllBalances { address }) => {
            // Query the contract state for the balance of the specified address
            let balances = config_read(deps.storage).load()?;
            let balance = balances.balances.get(&address).cloned().unwrap_or_default();
            to_binary(&balance)
        }
        _ => to_binary("Unsupported query request"),
    }
}
