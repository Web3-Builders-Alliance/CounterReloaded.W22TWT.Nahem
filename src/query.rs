use cosmwasm_std::{Deps, StdResult};

use crate::{
    msg::{GetCountResponse, GetResetResponse},
    state::STATE,
};

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
