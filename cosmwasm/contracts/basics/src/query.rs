use cosmwasm_std::StdResult;

use crate::msg::GreetResp;

pub fn greet(name: &str) -> StdResult<GreetResp> {
    let resp = GreetResp {
        message: format!("Hello {} from wasm contract", name),
    };

    Ok(resp)
}
