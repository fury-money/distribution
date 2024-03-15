use cosmwasm_std::{
    Addr, Api, Binary, CosmosMsg, Decimal, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Storage, Uint128, WasmMsg,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Staker {
    pub address: Addr,
    pub amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Stakers {
    pub stakers: Vec<Staker>,
}

pub fn store_stakers(storage: &mut dyn Storage, stakers: &Stakers) -> StdResult<()> {
    storage.set(b"stakers", &serde_json::to_vec(stakers)?);
    Ok(())
}

pub fn read_stakers(storage: &dyn Storage) -> StdResult<Stakers> {
    match storage.get(b"stakers") {
        Some(data) => Ok(serde_json::from_slice(&data)?),
        None => Ok(Stakers { stakers: vec![] }),
    }
}

pub fn distribute_rewards(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    amount: Uint128,
) -> StdResult<Response> {
    // Load stakers
    let stakers = read_stakers(deps.storage)?;

    // Perform reward distribution logic
    let num_stakers = stakers.stakers.len();
    if num_stakers == 0 {
        return Err(StdError::generic_err("No stakers found"));
    }
    let reward_per_staker = amount.checked_div(Uint128::from(num_stakers as u128))
        .ok_or_else(|| StdError::generic_err("Reward amount too small"))?;

    // Generate transfer messages for each staker
    let mut messages: Vec<CosmosMsg> = vec![];
    for staker in stakers.stakers.iter() {
        messages.push(CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: staker.address.clone(),
            msg: to_binary(&HandleMsg::DistributeReward {
                amount: reward_per_staker,
            })?,
            funds: vec![],
        }));
    }

    Ok(Response::new().add_messages(messages))
}

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _amount: Uint128,
) -> StdResult<Response> {
    // Store initial state if needed
    store_stakers(deps.storage, &Stakers { stakers: vec![] })?;

    Ok(Response::default())
}

pub fn handle(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> StdResult<Response> {
    match msg {
        HandleMsg::DistributeRewards { amount } => {
            // Only admin can distribute rewards
            if info.sender != ADMIN {
                return Err(StdError::unauthorized());
            }
            distribute_rewards(deps, _env, info, amount)
        }
        HandleMsg::AddStakers { stakers } => add_stakers(deps, info, stakers),
    }
}

pub fn add_stakers(deps: DepsMut, info: MessageInfo, stakers: Vec<Staker>) -> StdResult<Response> {
    // Only admin can add stakers
    if info.sender != ADMIN {
        return Err(StdError::unauthorized());
    }

    let mut stored_stakers = read_stakers(deps.storage)?;
    stored_stakers.stakers.extend(stakers);
    store_stakers(deps.storage, &stored_stakers)?;

    Ok(Response::default())
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    DistributeRewards { amount: Uint128 },
    AddStakers { stakers: Vec<Staker> },
}

const ADMIN: &str = "admin_contract_address"; // Set your admin address here

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockApi, MockStorage};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let info = mock_info("creator", &[]);
        let msg = instantiate(deps.as_mut(), env.clone(), info.clone(), Uint128::new(0)).unwrap();
        assert_eq!(0, msg.messages.len());

        // It should store the empty list of stakers
        let stakers: Stakers = read_stakers(deps.as_ref().storage).unwrap();
        assert_eq!(0, stakers.stakers.len());
    }

    #[test]
    fn add_stakers() {
        let mut deps = mock_dependencies(&[]);
        let env = mock_env();
        let info = mock_info(ADMIN, &[]);
        let addr1 = String::from("addr1");
        let addr2 = String::from("addr2");
        let amount1 = Uint128::new(100);
        let amount2 = Uint128::new(200);
        let staker1 = Staker {
            address: deps.api.addr_validate(&addr1).unwrap(),
            amount: amount1,
        };
        let staker2 = Staker {
            address: deps.api.addr_validate(&addr2).unwrap(),
            amount: amount2,
        };
        let msg = HandleMsg::AddStakers {
            stakers: vec![staker1.clone(), staker2.clone()],
        };
        let res = handle(deps.as_mut(), env.clone(), info.clone(), msg.clone()).unwrap();
        assert_eq!(0, res.messages.len());

        // It should store the stakers
        let stakers: Stakers = read_stakers(deps.as_ref().storage).unwrap();
        assert_eq!(2, stakers.stakers.len());
        assert_eq!(staker1, stakers.stakers[0]);
        assert_eq!(staker2, stakers.stakers[1]);
    }
}
