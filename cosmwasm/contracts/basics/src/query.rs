use cosmwasm_std::Deps;

use crate::{
    error::ContractResult,
    msg::{AdminsResp, GreetResp},
    state::ADMINS,
};

pub fn query_greeting(name: &str) -> ContractResult<GreetResp> {
    let resp = GreetResp {
        message: format!("Hello {} from wasm contract", name),
    };

    Ok(resp)
}

pub fn query_admins(deps: Deps) -> ContractResult<AdminsResp> {
    let admins = ADMINS.load(deps.storage)?;

    let resp = AdminsResp { admins };

    Ok(resp)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn query_greeting_given_a_name_returns_right_greeting() {
        let name = "Alice";

        let resp = query_greeting(name);

        let expected_resp = Ok(GreetResp {
            message: "Hello Alice from wasm contract".to_owned(),
        });

        assert_eq!(resp, expected_resp)
    }
}
