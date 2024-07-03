use cosmwasm_std::Addr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct InstantiateMsg {
    pub admins: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub enum QueryMsg {
    Greet { name: String },
    Admins {},
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ExecuteMsg {
    AddMembers { members: Vec<String> },
    Leave {},
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct GreetResp {
    pub message: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AdminsResp {
    pub admins: Vec<Addr>,
}
