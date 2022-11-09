use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub count: i32,
}

#[cw_serde]
pub enum ExecuteMsg {
    // Increment {},
    Reset { count: i32 },
    // Decrement {},
    Increment { amount: Option<i32> },
    Decrement { amount: Option<i32> },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCountResponse)]
    GetCount {},
    // GetResetCount returns the current reset count as a json-encoded number
    #[returns(GetResetResponse)]
    GetResetCount {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCountResponse {
    pub count: i32,
}

// We define a custom struct for each query reset count
#[cw_serde]
pub struct GetResetResponse {
    pub reset_count: i32,
}
