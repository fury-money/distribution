// state.rs

use cosmwasm_std::{Storage, StdResult};
use serde::Serialize;
use schemars::JsonSchema;
use serde::Deserialize;
use cosmwasm_storage::{singleton, singleton_read};

pub static ADMIN_KEY: &[u8] = b"admin";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub admin: String,
    pub balances: std::collections::BTreeMap<String, u128>,
}

pub fn config(storage: &mut dyn Storage) -> singleton::Singleton<State> {
    singleton(storage, b"config")
}

pub fn config_read(storage: &dyn Storage) -> singleton_read::SingletonRead<State> {
    singleton_read(storage, b"config")
}

pub fn get_admin(storage: &dyn Storage) -> StdResult<String> {
    ADMIN_KEY.load(storage)
}