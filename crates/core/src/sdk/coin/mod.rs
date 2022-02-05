use crate::sdk::numeric::{Dec, Uint128};
use lazy_static::lazy_static;
use regex::Regex;
use std::{
    convert::Into,
    ops::{Add, Div, Mul, Rem, Sub},
};
pub struct Coin {
    pub denom: String,
    pub amount: Uint128,
}

const NO_DENOM: &str = "";

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
            amount: Uint128::try_from(amount)?,
        })
    }
}

impl Add for Coin {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        if self.denom != rhs.denom {
            panic!("Cannot add coins with different denoms");
        }
        Coin {
            denom: self.denom,
            amount: self.amount + rhs.amount,
        }
    }
}

impl<A: Into<Uint128>> Add<A> for Coin {
    type Output = Self;
    fn add(self, rhs: A) -> Self {
        Coin {
            denom: self.denom,
            amount: self.amount + rhs.into(),
        }
    }
}

impl Sub for Coin {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        if self.denom != rhs.denom {
            panic!("Cannot subtract coins with different denoms");
        }
        Coin {
            denom: self.denom,
            amount: self.amount - rhs.amount,
        }
    }
}

impl<T: Into<Uint128>> Sub<T> for Coin {
    type Output = Self;
    fn sub(self, rhs: T) -> Self {
        Coin {
            denom: self.denom,
            amount: self.amount - rhs.into(),
        }
    }
}

impl<T: Into<Uint128>> Div<T> for Coin {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        Coin {
            denom: self.denom,
            amount: self.amount / rhs.into(),
        }
    }
}

impl<T: Into<Uint128>> Mul<T> for Coin {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Coin {
            denom: self.denom,
            amount: self.amount * rhs.into(),
        }
    }
}

impl<T: Into<Uint128>> Rem<T> for Coin {
    type Output = Self;
    fn rem(self, rhs: T) -> Self {
        Coin {
            denom: self.denom,
            amount: self.amount % rhs.into(),
        }
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
                    Uint128::try_from(*self).unwrap()
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
}
pub struct Coins {}
