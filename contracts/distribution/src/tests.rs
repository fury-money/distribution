// tests.rs

use cosmwasm_std::{from_binary, StdError, CosmosMsg, HumanAddr, Uint128, WasmMsg};
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

use crate::contract::{Contract, HandleMsg, InitMsg, QueryMsg};
use crate::msg::QueryResponse;
use crate::state::{config, ADMIN_KEY};

#[test]
fn proper_initialization() {
    let mut deps = mock_dependencies(20, &[]);

    let msg = InitMsg {
        initial_balances: vec![("addr1".to_string(), 100), ("addr2".to_string(), 50)],
    };
    let info = mock_info("creator", &[]);

    // we can just call .unwrap() to assert this was a success
    let res = contract::instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
    assert_eq!(0, res.messages.len());

    // it worked, let's query the state
    let res = contract::query(deps.as_ref(), mock_env(), QueryMsg::GetBalance {}).unwrap();
    let value: Vec<(String, u128)> = from_binary(&res).unwrap();
    assert_eq!(2, value.len());
    assert_eq!(100, value[0].1);
    assert_eq!(50, value[1].1);
}

#[test]
fn deposit_funds() {
    let mut deps = mock_dependencies(20, &[]);

    // Initialize the contract
    let init_msg = InitMsg {
        initial_balances: vec![("addr1".to_string(), 100)],
    };
    let init_info = mock_info("creator", &[]);
    contract::instantiate(deps.as_mut(), mock_env(), init_info, init_msg).unwrap();

    // Execute a deposit
    let deposit_msg = HandleMsg::Deposit {};
    let deposit_info = mock_info("addr2", &[coin("uscrt", 50)]);
    let res = contract::execute(deps.as_mut(), mock_env(), deposit_info, deposit_msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Query the updated balance
    let query_res = contract::query(deps.as_ref(), mock_env(), QueryMsg::GetBalance {}).unwrap();
    let value: Vec<(String, u128)> = from_binary(&query_res).unwrap();
    assert_eq!(2, value.len());
    assert_eq!(100, value[0].1);
    assert_eq!(50, value[1].1);
}

#[test]
fn distribute_funds() {
    let mut deps = mock_dependencies(20, &[]);

    // Initialize the contract
    let init_msg = InitMsg {
        initial_balances: vec![("addr1".to_string(), 100)],
    };
    let init_info = mock_info("creator", &[]);
    contract::instantiate(deps.as_mut(), mock_env(), init_info, init_msg).unwrap();

    // Execute a distribution
    let distribute_msg = HandleMsg::DistributeFunds {
        recipients: vec!["addr2".to_string(), "addr3".to_string()],
        amounts: vec![30, 20],
    };
    let distribute_info = mock_info("creator", &[]);
    let res = contract::execute(deps.as_mut(), mock_env(), distribute_info, distribute_msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Query the updated balance
    let query_res = contract::query(deps.as_ref(), mock_env(), QueryMsg::GetBalance {}).unwrap();
    let value: Vec<(String, u128)> = from_binary(&query_res).unwrap();
    assert_eq!(2, value.len());
    assert_eq!(70, value[0].1);  // 100 - 30
    assert_eq!(20, value[1].1);  // 0 + 20
}

#[test]
fn admin_change() {
    let mut deps = mock_dependencies(20, &[]);

    // Initialize the contract
    let init_msg = InitMsg {
        initial_balances: vec![("addr1".to_string(), 100)],
    };
    let init_info = mock_info("creator", &[]);
    contract::instantiate(deps.as_mut(), mock_env(), init_info, init_msg).unwrap();

    // Execute an admin change
    let new_admin = HumanAddr::from("new_admin");
    let admin_change_msg = HandleMsg::Admin {
        new_admin: new_admin.clone(),
    };
    let admin_change_info = mock_info("creator", &[]);
    let res = contract::execute(deps.as_mut(), mock_env(), admin_change_info, admin_change_msg).unwrap();
    assert_eq!(0, res.messages.len());

    // Query the updated admin
    let query_res = deps
        .querier
        .query(&QueryRequest::Wasm(WasmQuery::Raw {
            contract_addr: deps.api.addr_canonicalize("creator").unwrap(),
            key: ADMIN_KEY.into(),
        }))
        .unwrap();
    let queried_admin: String = from_binary(&query_res).unwrap();
    assert_eq!(new_admin, HumanAddr::from(queried_admin));
}

// Helper function to create a coin for testing
fn coin(denom: &str, amount: u128) -> cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
    cosmos_sdk_proto::cosmos::base::v1beta1::Coin {
        denom: denom.to_string(),
        amount: amount.to_string(),
    }
}
