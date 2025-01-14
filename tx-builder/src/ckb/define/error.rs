use ethereum_types::H160;
use thiserror::Error;

use common::types::tx_builder::{Amount, Epoch};

pub type CkbTxResult<T> = std::result::Result<T, CkbTxErr>;

#[derive(Error, Debug)]
pub enum CkbTxErr {
    #[error("Missing information for the first stake")]
    FirstStake,

    #[error("Invalid inaugration epoch, expected: {expected:?}, found: {found:?}")]
    InaugurationEpoch { expected: Epoch, found: Epoch },

    #[error("The stake/delegate amount is too large, wallet amount: {wallet_amount:?}, stake/delegate amount: {amount:?}")]
    ExceedWalletAmount {
        wallet_amount: Amount,
        amount:        Amount,
    },

    #[error("The stake/delegate amount is too large, total elect amount: {total_amount:?}, stake/delegate amount: {new_amount:?}")]
    ExceedTotalAmount {
        total_amount: Amount,
        new_amount:   Amount,
    },

    #[error("Invalid is_increase: {0}")]
    Increase(bool),

    #[error("Lack of capacity: {inputs_capacity:?} < {outputs_capacity:?}")]
    InsufficientCapacity {
        inputs_capacity:  u64,
        outputs_capacity: u64,
    },

    #[error(
        "The minted amount is too large, minted amount: {total_mint:?}, max supply: {max_supply:?}"
    )]
    ExceedMaxSupply {
        max_supply: Amount,
        total_mint: Amount,
    },

    #[error("Cell not found: {0}")]
    CellNotFound(String),

    #[error("Deserialize bls pub key error")]
    Deserialize,

    #[error("User's reward epoch not found")]
    RewardEpochNotFound,

    #[error("The minimum value of the current epoch should be 2")]
    EpochTooSmall,

    #[error("Stake amount not found in stack SMT")]
    StakeAmountNotFound(H160),
}
