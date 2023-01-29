use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{College, STATE};

#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:college";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = College { students: vec![] };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Increment { address } => execute::increment(deps, info, address),
    }
}

pub mod execute {
    use cosmwasm_std::{CosmosMsg, WasmMsg};

    use super::*;

    pub fn increment(
        _deps: DepsMut,
        info: MessageInfo,

        contract_address: String,
    ) -> Result<Response, ContractError> {
        let action: CosmosMsg<_> = CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: contract_address,
            msg: to_binary(&counter2::msg::ExecuteMsg::Increment {}).unwrap(),
            funds: info.funds,
        });

        Ok(Response::new()
            .add_attribute("action", "reset")
            .add_message(action))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetCount { address } => to_binary(&query::student(deps, address)?),
    }
}

pub mod query {

    use counter2::msg::GetCountResponse;

    use super::*;

    pub fn student(deps: Deps, address: String) -> StdResult<GetCountResponse> {
        let _state = STATE.load(deps.storage)?;

        let r = deps
            .querier
            .query_wasm_smart::<counter2::msg::GetCountResponse>(
                address,
                &counter2::msg::QueryMsg::GetCount {},
            )
            .unwrap();

        Ok(GetCountResponse { count: r.count })
    }
}
