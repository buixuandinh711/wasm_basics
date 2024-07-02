use cosmwasm_std::{Addr, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult};

use crate::{
    msg::{InstantiateMsg, QueryMsg},
    query as query_mod,
    state::ADMINS,
};

pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let admins: StdResult<Vec<Addr>> = msg
        .admins
        .into_iter()
        .map(|addr| deps.api.addr_validate(&addr))
        .collect();

    ADMINS.save(deps.storage, &admins?)?;

    Ok(Response::new())
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Greet { name } => {
            cosmwasm_std::to_json_binary(&query_mod::query_greeting(&name)?)
        }
        QueryMsg::Admins {} => cosmwasm_std::to_json_binary(&query_mod::query_admins(deps)?),
    }
}

#[allow(dead_code)]
pub fn execute(_deps: DepsMut, _env: Env, _info: MessageInfo, _msg: Empty) -> StdResult<Response> {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use cosmwasm_std::Addr;
    use cw_multi_test::{App, ContractWrapper, Executor, IntoBech32};

    use crate::msg::{AdminsResp, GreetResp};

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
                &InstantiateMsg { admins: vec![] },
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

    #[test]
    fn instantiate_with_no_address_stores_no_admin() {
        let mut app = App::default();

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let instantiate_msg = InstantiateMsg { admins: vec![] };

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &instantiate_msg,
                &[],
                "basics",
                None,
            )
            .unwrap();

        let query_msg = QueryMsg::Admins {};

        let resp: AdminsResp = app.wrap().query_wasm_smart(addr, &query_msg).unwrap();

        let expected_resp = AdminsResp { admins: vec![] };

        assert_eq!(resp, expected_resp);
    }

    #[test]
    fn instantiate_with_2_address_stores_2_admins() {
        let mut app = App::default();

        println!("{:?}", app.wrap().query_bonded_denom().unwrap());

        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let instantiate_msg = InstantiateMsg {
            admins: vec![
                "admin1".into_bech32().to_string(),
                "admin2".into_bech32().to_string(),
            ],
        };

        let addr = app
            .instantiate_contract(
                code_id,
                Addr::unchecked("owner"),
                &instantiate_msg,
                &[],
                "basics",
                None,
            )
            .unwrap();

        let query_msg = QueryMsg::Admins {};

        let resp: AdminsResp = app.wrap().query_wasm_smart(&addr, &query_msg).unwrap();

        let expected_resp = AdminsResp {
            admins: vec!["admin1".into_bech32(), "admin2".into_bech32()],
        };

        assert_eq!(resp, expected_resp)
    }
}
