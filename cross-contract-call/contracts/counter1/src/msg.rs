use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    Increment { address: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetCounterResponse)]
    GetCount { address: String },
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetCounterResponse {
    pub count_counter1: i32,
    pub count_counter2: i32,
}
