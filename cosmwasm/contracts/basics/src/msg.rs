use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum QueryMsg {
    Greet { name: String },
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct GreetResp {
    pub message: String,
}
