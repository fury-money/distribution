pub mod msg;
pub mod state;
pub mod query;

use cosmwasm_std::testing::{execute, instantiate}; // Import execute and instantiate directly from testing module
use crate::msg::HandleMsg;
use cosmwasm_std::StdError;
use crate::msg::QueryMsg;
use cosmwasm_std::from_binary;

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::Coin;

    fn default_init() -> msg::InitMsg {
        msg::InitMsg {
            initial_balances: vec![],
        }
    }

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
    
        let msg = default_init();
        let info = mock_info("creator", &[]);
    
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    #[test]
    fn deposit_works() {
        let mut deps = mock_dependencies();
    
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
		let mut deps = mock_dependencies();
	
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
		let mut deps = mock_dependencies();
	
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
		let mut deps = mock_dependencies();
	
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
		let mut deps = mock_dependencies();
	
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
