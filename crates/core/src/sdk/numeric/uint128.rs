use std::convert::{From, Into, TryFrom};
use std::ops::{Add, Div, Mul, Rem, Sub};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Uint128(u128);

pub type Int = Uint128;

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

impl TryFrom<&str> for Uint128 {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();
        if value.is_empty() {
            return Ok(Uint128::default());
        }
        let value = value.parse::<u128>().map_err(|_| "invalid number")?;
        Ok(Uint128(value))
    }
}

impl TryFrom<String> for Uint128 {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
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
    #[test]
    fn it_works() {}
}
