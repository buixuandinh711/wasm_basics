mod contract;
pub mod msg;
mod query;

use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult,
};
use msg::QueryMsg;

#[entry_point]
pub fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: Empty) -> StdResult<Response> {
    contract::instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn query(dep: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    contract::query(dep, env, msg)
}
