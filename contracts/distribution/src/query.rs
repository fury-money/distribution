use cosmwasm_std::{Deps, QueryRequest, QueryResponse, StdResult, to_binary};
use crate::state::config_read;
use crate::Contract;

pub fn handle_query(deps: Deps, request: QueryRequest) -> StdResult<QueryResponse> {
    match request {
        QueryRequest::Wasm(wasm_query) => contract_query(deps, wasm_query),
        _ => unsupported_query(),
    }
}

fn contract_query(deps: Deps, wasm_query: cosmwasm_std::WasmQuery) -> StdResult<QueryResponse> {
    match wasm_query {
        cosmwasm_std::WasmQuery::Smart { contract_addr, msg } => {
            match msg {
                ContractQuery::GetBalance { address } => get_balance(deps, address),
                // Add more query variants as needed
            }
        }
        _ => unsupported_query(),
    }
}

fn get_balance(deps: Deps, address: String) -> StdResult<QueryResponse> {
    let balances = Contract::query_balance(deps)?;

    let balance = balances.into_iter().find(|(addr, _)| addr == &address);

    match balance {
        Some((_addr, amount)) => Ok(QueryResponse::default().add_attribute("balance", to_binary(&amount)?)),
        None => Ok(QueryResponse::default().add_attribute("balance", to_binary(&0u128)?)),
    }
}

fn unsupported_query() -> StdResult<QueryResponse> {
    Ok(QueryResponse::default())
}

// Add more query variants as needed
#[derive(Clone, PartialEq, MessageInfo, Debug, Deserialize, Serialize, JsonSchema)]
pub enum ContractQuery {
    GetBalance { address: String },
    GetAdminAddress,
    // Add more query variants as needed
}
