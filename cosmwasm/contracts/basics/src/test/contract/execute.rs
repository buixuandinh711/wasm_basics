use std::vec;

use cosmwasm_std::Addr;
use cw_multi_test::{App, ContractWrapper, Executor, IntoBech32 as _};

use crate::{
    contract::{execute, instantiate, query},
    error::ContractError,
    msg::{AdminsResp, ExecuteMsg, InstantiateMsg, QueryMsg},
};

fn get_default_admins() -> Vec<Addr> {
    vec!["admin1".into_bech32(), "admin2".into_bech32()]
}

fn setup(app: &mut App, admins: Vec<Addr>, donation_denom: String) -> Addr {
    let code = ContractWrapper::new(execute, instantiate, query);
    let code_id = app.store_code(Box::new(code));

    let admins = admins.into_iter().map(|addr| addr.to_string()).collect();
    let init_msg = InstantiateMsg {
        admins,
        donation_denom,
    };

    let addr = app
        .instantiate_contract(
            code_id,
            Addr::unchecked("sender"),
            &init_msg,
            &[],
            "basics",
            None,
        )
        .unwrap();

    addr
}

#[test]
fn add_members_called_by_non_admin_return_unauthorized_error() {
    let default_admins = get_default_admins();
    let mut app = App::default();
    let contract_addr = setup(&mut app, default_admins.clone(), "fake_denom".to_owned());

    let execute_msg = ExecuteMsg::AddMembers { members: vec![] };

    let non_admin_sender = "not admin sender".into_bech32();

    let resp = app
        .execute_contract(non_admin_sender.clone(), contract_addr, &execute_msg, &[])
        .unwrap_err();

    let expected_resp: ContractError = ContractError::Unauthorized {
        sender: non_admin_sender,
    };

    assert_eq!(expected_resp, resp.downcast().unwrap(),);
}

#[test]
fn add_members_given_admin_address_return_duplicated_error() {
    let default_admins = get_default_admins();
    let mut app = App::default();
    let contract_addr = setup(&mut app, default_admins.clone(), "fake_denom".to_owned());

    let duplicated_admin = default_admins[1].clone();

    let execute_msg = ExecuteMsg::AddMembers {
        members: vec![duplicated_admin.to_string()],
    };

    let resp = app
        .execute_contract(default_admins[0].clone(), contract_addr, &execute_msg, &[])
        .unwrap_err();

    let expected_resp: ContractError = ContractError::DuplicatedMember {
        duplications: vec![duplicated_admin],
    };

    assert_eq!(expected_resp, resp.downcast().unwrap(),);
}

#[test]
fn add_members_given_new_members_updates_admins_list_properly() {
    let default_admins = get_default_admins();
    let mut app = App::default();
    let contract_addr = setup(&mut app, default_admins.clone(), "fake_denom".to_owned());

    let new_members = vec!["member1".into_bech32(), "member2".into_bech32()];

    let execute_msg = ExecuteMsg::AddMembers {
        members: new_members
            .clone()
            .into_iter()
            .map(|addr| addr.to_string())
            .collect(),
    };

    let sender = default_admins[0].clone();

    app.execute_contract(sender, contract_addr.clone(), &execute_msg, &[])
        .unwrap();

    let query_resp: AdminsResp = app
        .wrap()
        .query_wasm_smart(contract_addr.clone(), &QueryMsg::Admins {})
        .unwrap();

    let mut updated_admin = query_resp.admins;
    updated_admin.sort_unstable();

    let mut expected_admins = [&default_admins[..], &new_members[..]].concat();
    expected_admins.sort_unstable();

    assert_eq!(updated_admin, expected_admins);
}

#[test]
fn add_members_given_new_members_emit_event_properly() {
    let default_admins = get_default_admins();
    let mut app = App::default();
    let contract_addr = setup(&mut app, default_admins.clone(), "fake_denom".to_owned());

    let new_members = vec!["member1".into_bech32(), "member2".into_bech32()];

    let execute_msg = ExecuteMsg::AddMembers {
        members: new_members
            .clone()
            .into_iter()
            .map(|addr| addr.to_string())
            .collect(),
    };

    let sender = default_admins[0].clone();

    let execute_resp = app
        .execute_contract(sender, contract_addr.clone(), &execute_msg, &[])
        .unwrap();

    let events = execute_resp
        .events
        .into_iter()
        .find(|e| e.ty == "wasm-admin_added")
        .unwrap();

    let added_addresses: Vec<_> = events
        .attributes
        .into_iter()
        .filter_map(|attr| {
            if attr.key == "addr" {
                Some(attr.value)
            } else {
                None
            }
        })
        .collect();

    let expected_addresses: Vec<_> = new_members
        .into_iter()
        .map(|addr| addr.to_string())
        .collect();

    assert_eq!(added_addresses, expected_addresses);
}

#[test]
fn donate_given_token_divides_to_admins_properly() {
    let default_admins = get_default_admins();

    let sender = "sender".into_bech32();

    let mut app = App::new(|route, _, storage| {
        route
            .bank
            .init_balance(storage, &sender, cosmwasm_std::coins(1000, "eth"))
            .unwrap();
    });
    let contract_addr = setup(&mut app, default_admins.clone(), "eth".to_owned());

    let execute_msg = ExecuteMsg::Donate {};

    app.execute_contract(
        sender,
        contract_addr.clone(),
        &execute_msg,
        &[cosmwasm_std::coin(101, "eth")],
    )
    .unwrap();

    let admin1_balance = app
        .wrap()
        .query_balance(default_admins[0].clone(), "eth")
        .unwrap()
        .amount
        .u128();

    let admin2_balance = app
        .wrap()
        .query_balance(default_admins[0].clone(), "eth")
        .unwrap()
        .amount
        .u128();

    assert_eq!(admin1_balance, 50);
    assert_eq!(admin2_balance, 50);
}
