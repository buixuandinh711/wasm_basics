use cosmwasm_std::Addr;
use cw_multi_test::{App, ContractWrapper, Executor, IntoBech32 as _};

use crate::msg::AdminsResp;

use crate::{
    contract::{execute, instantiate, query},
    msg::{InstantiateMsg, QueryMsg},
};

#[test]
fn instantiate_with_no_address_stores_no_admin() {
    let mut app = App::default();

    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let instantiate_msg = InstantiateMsg {
        admins: vec![],
        donation_denom: "".to_owned(),
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
        donation_denom: "".to_owned(),
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
