use crate::sdk::numeric::{Dec, Uint128};
pub struct Coin {
    denom: String,
    amount: Uint128,
}

pub struct Coins {}

mod arithmetic;
pub use arithmetic::*;
