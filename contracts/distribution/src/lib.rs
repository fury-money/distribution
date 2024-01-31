use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

mod contract;
mod msg;
mod state;
mod query;

#[cfg(test)]
mod testing;

pub use crate::contract::Contract;
pub use crate::msg::{HandleMsg, InitMsg, QueryMsg};
pub use crate::state::{config, config_read, ADMIN_KEY};

// Implement the necessary CosmWasm entry points
#[no_mangle]
pub extern "C" fn init(deps: DepsMut, env: Env, info: MessageInfo, msg: InitMsg) -> StdResult<Response> {
    contract::instantiate(deps, env, info, msg)
}

#[no_mangle]
pub extern "C" fn handle(deps: DepsMut, env: Env, info: MessageInfo, msg: Binary) -> StdResult<Response> {
    let handle_msg: HandleMsg = cosmwasm_std::from_binary(&msg)?;
    contract::execute(deps, env, info, handle_msg)
}

#[no_mangle]
pub extern "C" fn query(deps: Deps, env: Env, msg: Binary) -> StdResult<Binary> {
    let query_msg: QueryMsg = cosmwasm_std::from_binary(&msg)?;
    query::query(deps, env, query_msg)
}
