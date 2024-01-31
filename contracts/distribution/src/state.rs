use cosmwasm_std::{ReadonlySingleton, StdResult, Storage};
use cosmwasm_storage::singleton;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct State {
    pub admin: String,
    pub balances: BTreeMap<String, u128>,
}

impl State {
    pub fn new(admin: String, balances: BTreeMap<String, u128>) -> Self {
        State { admin, balances }
    }
}

pub fn config(storage: &dyn Storage) -> singleton::Singleton<State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton(storage, CONFIG_KEY)
}
