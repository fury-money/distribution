use serde::{Deserialize, Serialize};
use cosmwasm_storage::{ReadonlySingleton, ReadonlySingletonCow, Singleton, SingletonCow};
use cosmwasm_std::{StdError, StdResult, Storage};

pub static ADMIN_KEY: &[u8] = b"admin_key";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct State {
    pub admin: String,
    pub balances: std::collections::BTreeMap<String, u128>,
}

impl State {
    pub fn new(admin: String, balances: std::collections::BTreeMap<String, u128>) -> Self {
        Self { admin, balances }
    }
}

pub fn config(storage: &mut dyn Storage) -> ReadonlySingleton<State> {
    ReadonlySingleton::new(storage, b"config")
}

pub fn get_admin(storage: &dyn Storage) -> StdResult<String> {
    ADMIN_KEY.load(storage)
}
