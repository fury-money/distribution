use cosmwasm_std::{
    entry_point, to_binary, Binary, Coin, CosmosMsg, Deps, DepsMut, Env, MessageInfo, QueryRequest, QueryResponse,
    Response, StdError, StdResult, Uint128, WasmMsg,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

const ADMIN_KEY: &[u8] = b"admin_key";

#[derive(Default, Serialize, Deserialize)]
pub struct State {
    pub admin: String,
    pub balances: BTreeMap<String, u128>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub enum HandleMsg {
    Deposit {},
    DistributeFunds { recipients: Vec<String>, amounts: Vec<u128> },
    Admin { new_admin: String },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub enum QueryMsg {
    GetBalance {},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub enum QueryAnswer {
    GetBalance { result: Vec<(String, u128)> },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub enum HandleAnswer {
    Deposit {},
    DistributeFunds {},
    Admin {},
}

#[entry_point]
pub fn instantiate(deps: DepsMut, _env: Env, info: MessageInfo, msg: InitMsg) -> StdResult<Response> {
    let mut balances = BTreeMap::new();
    for entry in msg.initial_balances {
        balances.insert(entry.address, entry.balance);
    }

    let state = State {
        admin: info.sender.to_string(),
        balances,
    };

    config(deps.storage).save(&state)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: HandleMsg) -> StdResult<Response> {
    match msg {
        HandleMsg::Deposit {} => deposit(deps, env, info),
        HandleMsg::DistributeFunds { recipients, amounts } => distribute_funds(deps, info, recipients, amounts),
        HandleMsg::Admin { new_admin } => try_change_admin(deps, info, new_admin),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::GetBalance {} => query_balance(deps),
    }
}

fn distribute_funds(
    deps: DepsMut,
    info: MessageInfo,
    recipients: Vec<String>,
    amounts: Vec<u128>,
) -> StdResult<Response> {
    let state = config_read(deps.storage).load()?;
    if info.sender != state.admin {
        return Err(StdError::Unauthorized {});
    }

    if recipients.len() != amounts.len() {
        return Err(StdError::generic_err("Invalid input: recipients and amounts length mismatch"));
    }

    let mut updated_balances = state.balances.clone();
    for (recipient, amount) in recipients.iter().zip(amounts) {
        let mut balance = updated_balances
            .entry(recipient.clone())
            .or_insert(0);
        *balance += amount;
    }

    config(deps.storage).save(&State {
        admin: state.admin,
        balances: updated_balances,
    })?;

    Ok(Response::default())
}

fn deposit(deps: DepsMut, _env: Env, info: MessageInfo) -> StdResult<Response> {
    let sent_amount = info.funds.iter().find(|coin| coin.denom == "uscrt").map(|coin| coin.amount.u128());

    if let Some(amount) = sent_amount {
        if amount <= 0 {
            return Err(StdError::generic_err("Invalid deposit amount"));
        }

        let mut state = config(deps.storage).load()?;
        state.balances
            .entry(info.sender.clone())
            .and_modify(|balance| *balance += amount)
            .or_insert(amount);
        config(deps.storage).save(&state)?;

        Ok(Response::new().add_attribute("action", "deposit"))
    } else {
        Err(StdError::generic_err("No funds sent with the deposit message"))
    }
}

fn try_change_admin(deps: DepsMut, info: MessageInfo, new_admin: String) -> StdResult<Response> {
    let mut state = config(deps.storage).load()?;
    if info.sender != state.admin {
        return Err(StdError::Unauthorized {});
    }

    state.admin = new_admin.clone();
    config(deps.storage).save(&state)?;

    deps.storage.set(ADMIN_KEY, new_admin.as_bytes());

    Ok(Response::default())
}

fn query_balance(deps: Deps) -> StdResult<QueryResponse> {
    let state = config_read(deps.storage).load()?;
    let balances: Vec<(String, u128)> = state
        .balances
        .iter()
        .map(|(addr, balance)| (addr.clone(), *balance))
        .collect();
    Ok(QueryResponse::new().add_attribute("result", to_binary(&QueryAnswer::GetBalance { result: balances })?))
}

fn config(storage: &mut dyn cosmwasm_std::Storage) -> Singleton<State> {
    singleton(storage, ADMIN_KEY)
}

fn config_read(storage: &dyn cosmwasm_std::Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, ADMIN_KEY)
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    fn default_init() -> InitMsg {
        InitMsg {
            initial_balances: vec![],
        }
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);
    
        let msg = default_init();
        let info = mock_info("creator", &[]);
    
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn deposit_works() {
        let mut deps = mock_dependencies(&[]);
    
        let msg = default_init();
        let info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    
        let info = mock_info("user", &[Coin::new(100, "uscrt")]);
        let msg = HandleMsg::Deposit {};
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.attributes[0].key, "action");
        assert_eq!(res.attributes[0].value, "deposit");
    }

    #[test]
    fn deposit_with_no_funds_results_in_error() {
        let mut deps = mock_dependencies(&[]);
    
        let msg = default_init();
        let info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    
        let info = mock_info("user", &[]);
        let msg = HandleMsg::Deposit {};
    
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert_eq!(res.unwrap_err(), StdError::generic_err("No funds sent with the deposit message"));
    }

    #[test]
    fn distribute_funds_works() {
        let mut deps = mock_dependencies(&[]);
    
        let msg = default_init();
        let info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    
        // Deposit some funds
        let info = mock_info("user1", &[Coin::new(100, "uscrt")]);
        let msg = HandleMsg::Deposit {};
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    
        // Distribute funds
        let info = mock_info("creator", &[]);
        let recipients = vec!["user1".to_string()];
        let amounts = vec![50u128];
        let msg = HandleMsg::DistributeFunds { recipients, amounts };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.attributes[0].key, "action");
        assert_eq!(res.attributes[0].value, "distribute_funds");
    }

    #[test]
    fn try_change_admin_works() {
        let mut deps = mock_dependencies(&[]);
    
        let msg = default_init();
        let info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    
        let info = mock_info("creator", &[]);
        let new_admin = "new_admin".to_string();
        let msg = HandleMsg::Admin { new_admin: new_admin.clone() };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    
        assert_eq!(res.attributes.len(), 1);
        assert_eq!(res.attributes[0].key, "action");
        assert_eq!(res.attributes[0].value, "try_change_admin");
    }

    #[test]
    fn query_balance_works() {
        let mut deps = mock_dependencies(&[]);
    
        let msg = default_init();
        let info = mock_info("creator", &[]);
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    
        let info = mock_info("user1", &[Coin::new(100, "uscrt")]);
        let msg = HandleMsg::Deposit {};
        execute(deps.as_mut(), mock_env(), info, msg).unwrap();
    
        let query_msg = QueryMsg::GetBalance {};
        let query_res = query(deps.as_ref(), mock_env(), query_msg).unwrap();
        let balance: Vec<(String, u128)> = from_binary(&query_res).unwrap();
    
        assert_eq!(balance.len(), 1);
        assert_eq!(balance[0].0, "user1");
        assert_eq!(balance[0].1, 100u128);
    }
}