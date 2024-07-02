use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};

use crate::{msg::QueryMsg, query as query_mod};

pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Empty,
) -> StdResult<Response> {
    Ok(Response::new())
}

pub fn query(_dep: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Greet { name } => cosmwasm_std::to_json_binary(&query_mod::greet(&name)?),
    }
}

#[allow(dead_code)]
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor};

    use crate::msg::GreetResp;

    use super::*;

    #[test]
    fn query_greet_return_right_greet_with_name() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &Empty {},
                &[],
                "basics",
                None,
            )
            .unwrap();

        let msg = QueryMsg::Greet {
            name: "Bob".to_owned(),
        };

        let resp: GreetResp = app.wrap().query_wasm_smart(addr, &msg).unwrap();

        let expected_resp = GreetResp {
            message: "Hello Bob from wasm contract".to_owned(),
        };

        assert_eq!(resp, expected_resp)
    }
}
