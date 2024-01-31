use cosmwasm_std::{Deps, Env, StdResult, BankQuery, WasmQuery, to_binary};
use cosmwasm_std::QueryRequest;
use crate::state::config_read;
use crate::contract;  // Ensure this line is present


pub fn handle_query<C>(deps: Deps, env: Env, request: QueryRequest<C>) -> StdResult<Vec<u8>> {
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