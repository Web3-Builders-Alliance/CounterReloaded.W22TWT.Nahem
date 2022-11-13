use cosmwasm_std::{DepsMut, MessageInfo, Response};

use crate::{state::STATE, ContractError};

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
