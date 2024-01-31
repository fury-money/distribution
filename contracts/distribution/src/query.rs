use cosmwasm_std::{Deps, QueryRequest, StdResult};
use cosmwasm_std::WasmQuery;
use crate::state::config_read;
use cosmwasm_std::BankQuery;
use cosmwasm_std::to_binary;
use crate::contract;  // Ensure this line is present

// Your other imports and code...



pub fn handle_query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
	query(deps, env, msg)  
    match request {
        QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }) => {
            // Dispatch the query to the contract's query method
            contract::query(deps, msg)
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
