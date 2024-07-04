use std::collections::HashSet;

use cosmwasm_std::{Addr, DepsMut, Event, MessageInfo, Response, StdResult};

use crate::{error::ContractError, state::ADMINS};

pub fn add_members(
    deps: DepsMut,
    info: MessageInfo,
    new_admins: Vec<String>,
) -> Result<Response, ContractError> {
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

pub fn leave(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    check_admin_permission(&deps, &info)?;

    let mut cur_admin = ADMINS.load(deps.storage)?;

    let sender = info.sender;

    cur_admin.retain(|addr| addr != sender);

    let event = Event::new("admin_leaved").add_attribute("addr", sender.to_string());

    let resp = Response::new().add_event(event);

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
