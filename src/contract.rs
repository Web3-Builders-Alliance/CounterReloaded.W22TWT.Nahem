#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetCountResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:counter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        count: msg.count,
        owner: info.sender.clone(),
        reset_count: 0,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("count", msg.count.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Reset { count } => execute::reset(deps, info, count),
        ExecuteMsg::Increment { amount } => execute::increment(deps, amount),
        ExecuteMsg::Decrement { amount } => execute::decrement(deps, amount),
    }
}

pub mod execute {
    use super::*;

    pub fn reset(
        deps: DepsMut,
        info: MessageInfo,
        count: Option<i32>,
    ) -> Result<Response, ContractError> {
        let state = STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            if info.sender != state.owner {
                return Err(ContractError::Unauthorized {});
            }
            state.count = count.unwrap_or(0);
            state.reset_count += 1;
            Ok(state)
        })?;
        Ok(Response::new()
            .add_attribute("action", "reset")
            .add_attribute("count", state.count.to_string()))
    }

    pub fn increment(deps: DepsMut, amount: Option<i32>) -> Result<Response, ContractError> {
        let state = STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.count += amount.unwrap_or(1);
            Ok(state)
        })?;

        Ok(Response::new()
            .add_attribute("action", "increment")
            .add_attribute("count", state.count.to_string()))
    }

    pub fn decrement(deps: DepsMut, amount: Option<i32>) -> Result<Response, ContractError> {
        let state = STATE.update(deps.storage, |mut state| -> Result<_, ContractError> {
            state.count -= amount.unwrap_or(1);
            Ok(state)
        })?;

        Ok(Response::new()
            .add_attribute("action", "decrement")
            .add_attribute("count", state.count.to_string()))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount {} => to_binary(&query::count(deps)?),
        QueryMsg::GetResetCount {} => to_binary(&query::reset_count(deps)?),
    }
}

pub mod query {
    use crate::msg::GetResetResponse;

    use super::*;

    pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetCountResponse { count: state.count })
    }

    pub fn reset_count(deps: Deps) -> StdResult<GetResetResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetResetResponse {
            reset_count: state.reset_count,
        })
    }
}
