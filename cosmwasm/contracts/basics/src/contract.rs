use cosmwasm_std::{Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::{
    error::ContractResult,
    execute as execute_mod,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    query as query_mod,
    state::ADMINS,
};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> ContractResult<Response> {
    let admins: StdResult<Vec<Addr>> = msg
        .admins
        .into_iter()
        .map(|addr| deps.api.addr_validate(&addr))
        .collect();

    ADMINS.save(deps.storage, &admins?)?;

    Ok(Response::new())
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> ContractResult<Binary> {
    match msg {
        QueryMsg::Greet { name } => Ok(cosmwasm_std::to_json_binary(&query_mod::query_greeting(
            &name,
        )?)?),
        QueryMsg::Admins {} => Ok(cosmwasm_std::to_json_binary(&query_mod::query_admins(
            deps,
        )?)?),
    }
}

pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult<Response> {
    match msg {
        ExecuteMsg::AddMembers { members } => execute_mod::add_members(deps, info, members),
        ExecuteMsg::Leave {} => execute_mod::leave(deps, info),
    }
}