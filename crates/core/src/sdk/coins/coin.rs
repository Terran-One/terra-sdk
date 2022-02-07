use std::str::FromStr;

use crate::sdk::numeric::{Dec, Uint128};
use lazy_static::lazy_static;
use regex::Regex;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Coin {
    pub denom: String,
    pub amount: Uint128,
}

pub const NO_DENOM: &str = "";

impl Coin {
    pub fn new(denom: impl Into<String>, amount: impl Into<Uint128>) -> Self {
        Coin {
            denom: denom.into(),
            amount: amount.into(),
        }
    }

    pub fn parse(coin_str: &str) -> Result<Self, String> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^([0-9]+)([a-zA-Z]+)$").unwrap();
        }
        let caps = RE
            .captures(coin_str)
            .ok_or_else(|| format!("Invalid coin string: {}", coin_str))?;
        let denom = caps.get(2).map_or(NO_DENOM, |m| m.as_str());
        let amount = caps
            .get(1)
            .ok_or_else(|| format!("Invalid coin string: {}", coin_str))?
            .as_str();
        Ok(Coin {
            denom: denom.to_string(),
            amount: Uint128::from_str(amount)?,
        })
    }
}

impl FromStr for Coin {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Coin::parse(s)
    }
}

impl std::fmt::Display for Coin {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}{}", self.amount, self.denom)
    }
}

pub trait CoinMacroExprInput {
    fn to_coin(&self) -> Coin;
}

impl CoinMacroExprInput for &str {
    fn to_coin(&self) -> Coin {
        Coin::parse(self).unwrap()
    }
}

impl CoinMacroExprInput for String {
    fn to_coin(&self) -> Coin {
        Coin::parse(self).unwrap()
    }
}

pub trait CoinMacroAmount {
    fn parse_uint(&self) -> Uint128;
}

macro_rules! impl_coin_macro_amount_unsigned {
    ($($t:ty),*) => {
        $(
            impl CoinMacroAmount for $t {
                fn parse_uint(&self) -> Uint128 {
                    Uint128::from(*self)
                }
            }
        )*
    };
}

macro_rules! impl_coin_macro_amount_signed {
    ($($t:ty),*) => {
        $(
            impl CoinMacroAmount for $t {
                fn parse_uint(&self) -> Uint128 {
                    if self < &0 {
                        panic!("coin!(denom, amount) has invalid amount {} < 0", self);
                    }
                    crate::Uint128::try_from(*self).unwrap()
                }
            }
        )*
    };
}

impl_coin_macro_amount_unsigned!(usize, u8, u16, u32, u64, u128, Uint128);
impl_coin_macro_amount_signed!(isize, i8, i16, i32, i64, i128);

#[macro_export]
macro_rules! coin {
    ($input:expr) => {
        CoinMacroExprInput::to_coin(&$input)
    };
    ($denom:expr, $amount:expr) => {
        Coin::new($denom, CoinMacroAmount::parse_uint(&$amount))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn macro_works() {
        let a = coin!("uluna", 1);
        let b = Coin::new("uluna", 1u128);
    }

    #[test]
    fn it_serializes() {
        let a = coin!("uluna", 1);
        let json = serde_json::to_string(&a).unwrap();
        println!("{}", json);
    }
}
