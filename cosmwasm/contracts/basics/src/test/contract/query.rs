use cosmwasm_std::Addr;
use cw_multi_test::{App, ContractWrapper, Executor};

use crate::msg::GreetResp;

use crate::{
    contract::{execute, instantiate, query},
    msg::{InstantiateMsg, QueryMsg},
};

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
