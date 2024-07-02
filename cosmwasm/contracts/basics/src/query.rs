use cosmwasm_std::StdResult;

use crate::msg::GreetResp;

pub fn greet(name: &str) -> StdResult<GreetResp> {
    let resp = GreetResp {
        message: format!("Hello {} from wasm contract", name),
    };

    Ok(resp)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn greet_return_right_name() {
        let name = "Alice";

        let resp = greet(name);

        let expected_resp = Ok(GreetResp {
            message: "Hello Alice from wasm contract".to_owned(),
        });

        assert_eq!(resp, expected_resp)
    }
}
