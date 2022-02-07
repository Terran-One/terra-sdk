use serde::{Deserialize, Serialize};
use std::convert::{From, Into, TryFrom};
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::str::FromStr;
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub struct Uint128(u128);

impl Serialize for Uint128 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Uint128 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Uint128::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl Uint128 {
    pub fn new<T: Into<Uint128>>(value: T) -> Self {
        Uint128(value.into().u128())
    }

    pub fn u128(&self) -> u128 {
        self.0
    }
}

impl std::default::Default for Uint128 {
    fn default() -> Self {
        Uint128(0)
    }
}

impl FromStr for Uint128 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.is_empty() {
            return Ok(Uint128::default());
        }
        let value = s.parse::<u128>().map_err(|_| "invalid number")?;
        Ok(Uint128(value))
    }
}

macro_rules! impl_from_unsigned {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Uint128 {
                fn from(value: $t) -> Self {
                    Uint128(value as u128)
                }
            }
        )*
    };
}

impl_from_unsigned!(usize, u8, u16, u32, u64, u128);

macro_rules! impl_try_from_signed {
    ($($t:ty),*) => {
        $(
            impl TryFrom<$t> for Uint128 {
                type Error = ();
                fn try_from(value: $t) -> Result<Self, Self::Error> {
                    if value < 0 {
                        Err(())
                    } else {
                        Ok(Uint128(value as u128))
                    }
                }
            }
        )*
    };
}

impl_try_from_signed!(isize, i8, i16, i32, i64, i128);

impl<T> Add<T> for Uint128
where
    T: Into<Self>,
{
    type Output = Self;

    fn add(self, other: T) -> Self {
        Uint128(self.0 + other.into().0)
    }
}

impl<T> Sub<T> for Uint128
where
    T: Into<Self>,
{
    type Output = Self;

    fn sub(self, other: T) -> Self {
        Uint128(self.0 - other.into().0)
    }
}

impl<T> Mul<T> for Uint128
where
    T: Into<Self>,
{
    type Output = Self;

    fn mul(self, other: T) -> Self {
        Uint128(self.0 * other.into().0)
    }
}

impl<T> Div<T> for Uint128
where
    T: Into<Self>,
{
    type Output = Self;

    fn div(self, other: T) -> Self {
        Uint128(self.0 / other.into().0)
    }
}

impl<T> Rem<T> for Uint128
where
    T: Into<Self>,
{
    type Output = Self;

    fn rem(self, other: T) -> Self {
        Uint128(self.0 % other.into().0)
    }
}

impl Into<u128> for Uint128 {
    fn into(self) -> u128 {
        self.0
    }
}

impl std::fmt::Display for Uint128 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use serde_json::*;

    #[test]
    fn it_serializes() {
        let value = Uint128::from_str("1000000").unwrap();
        let serialized = serde_json::to_string(&value).unwrap();
        println!("{}", serialized);
    }
}
