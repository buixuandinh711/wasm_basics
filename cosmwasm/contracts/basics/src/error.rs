use cosmwasm_std::{Addr, StdError};
use cw_utils::PaymentError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("Sender {sender} is not contract admin")]
    Unauthorized { sender: Addr },

    #[error("Sender {:?} is not contract admin", duplications)]
    DuplicatedMember { duplications: Vec<Addr> },

    #[error("{0:?}")]
    InvalidDonation(#[from] PaymentError),

    #[error("{0}")]
    StdError(#[from] StdError),
}

pub type ContractResult<T> = Result<T, ContractError>;
