use std::collections::HashSet;

use cosmwasm_std::{Addr, BankMsg, DepsMut, Env, Event, MessageInfo, Response, StdResult};

use crate::{
    error::{ContractError, ContractResult},
    state::{ADMINS, DONATION_DENOM},
};

pub fn add_members(
    deps: DepsMut,
    info: MessageInfo,
    new_admins: Vec<String>,
) -> ContractResult<Response> {
    check_admin_permission(&deps, &info)?;

    let mut cur_admin: HashSet<_> = ADMINS.load(deps.storage)?.into_iter().collect();

    let new_admins: StdResult<Vec<Addr>> = new_admins
        .into_iter()
        .map(|addr| deps.api.addr_validate(&addr))
        .collect();

    let mut duplicated_addr = vec![];

    let mut events = Event::new("admin_added");

    for addr in new_admins.unwrap() {
        if cur_admin.contains(&addr) {
            duplicated_addr.push(addr);
        } else {
            cur_admin.insert(addr.clone());
            events = events.add_attribute("addr", addr);
        }
    }

    if !duplicated_addr.is_empty() {
        return Err(ContractError::DuplicatedMember {
            duplications: duplicated_addr,
        });
    }

    ADMINS.save(deps.storage, &cur_admin.into_iter().collect())?;

    let resp = Response::new().add_event(events);

    Ok(resp)
}

pub fn leave(deps: DepsMut, info: MessageInfo) -> ContractResult<Response> {
    check_admin_permission(&deps, &info)?;

    let mut cur_admin = ADMINS.load(deps.storage)?;

    let sender = info.sender;

    cur_admin.retain(|addr| addr != sender);

    let event = Event::new("admin_leaved").add_attribute("addr", sender.to_string());

    let resp = Response::new().add_event(event);

    Ok(resp)
}

pub fn donate(deps: DepsMut, env: Env, info: MessageInfo) -> ContractResult<Response> {
    let donation_denom = DONATION_DENOM.load(deps.storage)?;
    let admins = ADMINS.load(deps.storage)?;

    let donation = cw_utils::must_pay(&info, &donation_denom)?.u128();

    let left_donation = deps
        .querier
        .query_balance(&env.contract.address, &donation_denom)?
        .amount
        .u128();

    let donation_per_admin = (left_donation) / (admins.len() as u128);

    let messages = admins.into_iter().map(|addr| BankMsg::Send {
        to_address: addr.to_string(),
        amount: cosmwasm_std::coins(donation_per_admin, &donation_denom),
    });

    let resp = Response::new()
        .add_messages(messages)
        .add_attribute("donate", donation.to_string());

    Ok(resp)
}

fn check_admin_permission(deps: &DepsMut, info: &MessageInfo) -> Result<(), ContractError> {
    let cur_admin = ADMINS.load(deps.storage)?;

    let sender = &info.sender;

    if !cur_admin.contains(&sender) {
        return Err(ContractError::Unauthorized {
            sender: sender.clone(),
        });
    }

    Ok(())
}
