use cosmwasm_std::{Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::{
    error::ContractResult,
    execute as execute_mod,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    query as query_mod,
    state::{ADMINS, DONATION_DENOM},
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
    DONATION_DENOM.save(deps.storage, &msg.donation_denom)?;

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
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> ContractResult<Response> {
    match msg {
        ExecuteMsg::AddMembers { members } => execute_mod::add_members(deps, info, members),
        ExecuteMsg::Leave {} => execute_mod::leave(deps, info),
        ExecuteMsg::Donate {} => execute_mod::donate(deps, env, info),
    }
}
