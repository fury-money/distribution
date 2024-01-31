use cosmwasm_std::{
    entry_point, from_binary, to_binary, Binary, BankMsg, Coin, CosmosMsg, Decimal, Deps, DepsMut, Env,
    MessageInfo, QueryResponse, Response, StdResult, Uint128, WasmMsg,
};
use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{config, config_read, State};
use std::collections::BTreeMap;
use cosmwasm_std::StdError;
use cosmwasm_std::Storage;


const ADMIN_KEY: &[u8] = b"admin_key";



// Define the initial state struct
#[derive(Default)]
pub struct Contract;

// Implement the contract methods
impl Contract {
    pub fn instantiate(deps: DepsMut, _env: Env, info: MessageInfo, msg: InitMsg) -> StdResult<Response> {
		let balances: BTreeMap<String, u128> = msg
			.initial_balances
			.into_iter()
			.collect(); // Convert Vec to BTreeMap
	
		let state = State {
			admin: info.sender.to_string(),
			balances,
		};
	
		config(deps.storage).save(&state)?;
	
		Ok(Response::default())
	}

    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: HandleMsg,
    ) -> StdResult<Response> {
        match msg {
            HandleMsg::Deposit {} => deposit(deps, env, info),
            HandleMsg::DistributeFunds { recipients, amounts } => distribute_funds(deps, info, recipients, amounts),
            HandleMsg::Admin { new_admin } => try_change_admin(deps, info, new_admin),
        }
    }

    pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
		match msg {
			QueryMsg::GetBalance {} => Ok(QueryResponse::default().with_result(query_balance(deps)?)),
		}
	}
}

fn distribute_funds(
    deps: DepsMut,
    info: MessageInfo,
    recipients: Vec<String>,
    amounts: Vec<u128>,
) -> StdResult<Response> {
    // Ensure the sender is the admin
    let state = config_read(deps.storage).load()?;
    if info.sender != state.admin {
        return Err(StdError::GenericErr {
            msg: "Unauthorized".to_string(),
        });
    }

    // Ensure the number of recipients matches the number of amounts
    if recipients.len() != amounts.len() {
        return Err(StdError::generic_err("Invalid input: recipients and amounts length mismatch"));
    }

    // Distribute funds
    let mut updated_balances = state.balances.clone();
    for (recipient, amount) in recipients.iter().zip(amounts) {
        let mut balance = updated_balances
            .entry(recipient.clone())
            .or_insert(0);
        *balance += amount;
    }

    // Update state
    config(deps.storage).save(&State {
        admin: state.admin,
        balances: updated_balances,
    })?;

    Ok(Response::default())
}

fn deposit(deps: DepsMut, _env: Env, info: MessageInfo) -> StdResult<Response> {
    let sent_amount = info.funds.iter().find(|coin| coin.denom == "uscrt").map(|coin| coin.amount.u128());

    if let Some(amount) = sent_amount {
        // Validate the deposit amount (customize this based on your requirements)
        if amount <= 0 {
            return Err(StdError::generic_err("Invalid deposit amount"));
        }

        // Update the contract state
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

fn load_admin_key(storage: &dyn Storage) -> Vec<u8> {
    storage.get(ADMIN_KEY).unwrap_or_default()
}

fn try_change_admin(deps: DepsMut, info: MessageInfo, new_admin: String) -> StdResult<Response> {
    // Ensure the sender is the current admin
    let mut state = config(deps.storage).load()?;
    if info.sender != state.admin {
        return Err(StdError::GenericErr {
            msg: "Unauthorized".to_string(),
        });
    }

    // Update admin
    state.admin = new_admin.clone();
    config(deps.storage).save(&state)?;

    // Save the new admin in a separate key (optional)
    ADMIN_KEY.save(deps.storage, new_admin.as_bytes())?;

    Ok(Response::default())
}

fn query_balance(deps: Deps) -> StdResult<Vec<(String, u128)>> {
    let state = config_read(deps.storage).load()?;
    let balances: Vec<(String, u128)> = state
        .balances
        .iter()
        .map(|(addr, balance)| (addr.clone(), *balance))
        .collect();
    Ok(balances)
}
