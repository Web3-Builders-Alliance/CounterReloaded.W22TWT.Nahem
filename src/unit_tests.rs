#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::msg::{ExecuteMsg, GetCountResponse, GetResetResponse, InstantiateMsg, QueryMsg};
    use crate::ContractError;

    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // increment by 1
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment { amount: None };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);

        // increment by 5
        let msg = ExecuteMsg::Increment { amount: Some(5) };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(23, value.count);
    }

    #[test]
    fn decrement() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // decrement by 1
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Decrement { amount: None };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        // should decrease counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(16, value.count);

        // decrement by 5
        let msg = ExecuteMsg::Decrement { amount: Some(5) };
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should decrease counter by 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(11, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // "anyone" cannot reset the counter, only "creator" can
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: Some(5) };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // let's reset to 5 as "creator"
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: Some(5) };
        let _res = execute(deps.as_mut(), mock_env(), auth_info.clone(), msg).unwrap();

        // count should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let count_value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(5, count_value.count);

        // reset_count should now be 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetResetCount {}).unwrap();
        let reset_count_value: GetResetResponse = from_binary(&res).unwrap();
        assert_eq!(1, reset_count_value.reset_count);

        // let's reset again without passing the count. reset_count should now be 2
        let msg = ExecuteMsg::Reset { count: None };
        execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();
        let bin_reset_count = query(deps.as_ref(), mock_env(), QueryMsg::GetResetCount {}).unwrap();
        let res_reset_count_value: GetResetResponse = from_binary(&bin_reset_count).unwrap();
        assert_eq!(2, res_reset_count_value.reset_count);

        // count should be 0
        let bin_count = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let res_count_value: GetCountResponse = from_binary(&bin_count).unwrap();
        assert_eq!(0, res_count_value.count);
    }
}
