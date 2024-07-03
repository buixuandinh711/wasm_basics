mod contract;
mod error;
mod execute;
mod msg;
mod query;
mod state;
mod test;

use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response};
use error::ContractResult;
use msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResult<Response> {
    contract::instantiate(deps, env, info, msg)
}

#[entry_point]
pub fn query(dep: Deps, env: Env, msg: QueryMsg) -> ContractResult<Binary> {
    contract::query(dep, env, msg)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult<Response> {
    contract::execute(deps, env, info, msg)
}
