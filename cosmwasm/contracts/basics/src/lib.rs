use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct QueryResp {
    message: String,
}

#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

#[entry_point]
pub fn query(_dep: Deps, _env: Env, _msg: Empty) -> StdResult<Binary> {
    let resp = QueryResp {
        message: "World hellos you".to_owned(),
    };

    cosmwasm_std::to_json_binary(&resp)
}
