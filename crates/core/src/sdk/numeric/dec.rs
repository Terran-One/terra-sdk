use std::convert::{From, Into, TryFrom};
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::str::FromStr;

use num::{Integer, ToPrimitive};

/// Type for representing decimal numbers as they are used in
/// Cosmos SDK (18 digits of precision).
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Dec(i128);

pub const DEC_PRECISION: u32 = 18;
pub const DEC_ONE: i128 = 10i128.pow(DEC_PRECISION);

pub enum Sign {
    Positive,
    Negative,
}

pub trait IntoDec {
    fn into_dec(self) -> Dec;
}

impl Dec {
    pub fn new<T: Into<i128>>(value: T) -> Self {
        Dec(value.into())
    }

    pub fn zero() -> Self {
        Dec(0)
    }

    pub fn one() -> Self {
        Dec(DEC_ONE)
    }

    pub fn sign(&self) -> i64 {
        if self.0 < 0 {
            1
        } else {
            -1
        }
    }

    pub fn integer(&self) -> i64 {
        self.0.div_floor(&DEC_ONE).to_i64().unwrap()
    }

    pub fn fraction(&self) -> f64 {
        self.0.rem(&DEC_ONE).to_f64().unwrap().div(DEC_ONE as f64)
    }

    pub fn as_f64(&self) -> f64 {
        self.clone().into()
    }
}

macro_rules! impl_from_primitive {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Dec {
                fn from(value: $t) -> Self {
                    Dec::new(value as i128 * DEC_ONE)
                }
            }
        )*
    };
}

impl_from_primitive!(usize);
impl_from_primitive!(u8);
impl_from_primitive!(u16);
impl_from_primitive!(u32);
impl_from_primitive!(isize);
impl_from_primitive!(i8);
impl_from_primitive!(i16);
impl_from_primitive!(i32);

impl Into<f64> for Dec {
    fn into(self) -> f64 {
        let integer = self.integer();
        let fraction = self.fraction();
        let mut result;
        if integer < 0 {
            result = -fraction;
        } else {
            result = fraction;
        }
        result += integer as f64;
        return result;
    }
}

impl<T: Into<Dec>> Add<T> for Dec {
    type Output = Self;
    fn add(self, rhs: T) -> Self {
        Dec(self.0 + rhs.into().0)
    }
}

impl<T: Into<Dec>> Sub<T> for Dec {
    type Output = Self;
    fn sub(self, rhs: T) -> Self {
        Dec(self.0 - rhs.into().0)
    }
}

impl<T: Into<Dec>> Mul<T> for Dec {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let rhs = rhs.into();
        Dec(self.0 * rhs.0 / DEC_ONE)
    }
}

impl<T: Into<Dec>> Div<T> for Dec {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        Dec(self.0 / rhs.into().0)
    }
}

impl FromStr for Dec {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('.').collect::<Vec<_>>();
        if parts.len() > 2 {
            return Err(format!("invalid decimal string: {}", s));
        }
        if parts.len() == 1 {
            let integer = parts[0]
                .parse::<i128>()
                .map_err(|_| format!("invalid decimal string: {}", s))?;
            return Ok(Dec(integer * DEC_ONE));
        } else {
            let integer = parts[0]
                .parse::<i128>()
                .map_err(|_| format!("invalid decimal string: {}", s))?;
            let mut fraction = parts[1]
                .parse::<i128>()
                .map_err(|_| format!("invalid decimal string: {}", s))?;
            fraction = fraction * 10i128.pow(DEC_PRECISION - parts[1].len() as u32);
            return Ok(Dec(integer + fraction));
        }
    }
}

impl std::fmt::Display for Dec {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_f64())
    }
}

trait DecMacroInput {
    fn into_dec(&self) -> Dec;
}

macro_rules! impl_dec_macro_input {
    ($($t:ty),*) => {
        $(
            impl DecMacroInput for $t {
                fn into_dec(&self) -> Dec {
                    Dec::new(*self as i128 * DEC_ONE)
                }
            }
        )*
    };
}

impl_dec_macro_input!(usize, u8, u16, u32, u64, isize, i8, i16, i32, i64);

impl DecMacroInput for &str {
    fn into_dec(&self) -> Dec {
        Dec::from_str(self).unwrap()
    }
}

impl DecMacroInput for String {
    fn into_dec(&self) -> Dec {
        Dec::from_str(self).unwrap()
    }
}

impl DecMacroInput for f32 {
    fn into_dec(&self) -> Dec {
        Dec::from_str(self.to_string().as_str()).unwrap()
    }
}

impl DecMacroInput for f64 {
    fn into_dec(&self) -> Dec {
        Dec::from_str(self.to_string().as_str()).unwrap()
    }
}

#[macro_export]
macro_rules! dec {
    ($e:expr) => {
        DecMacroInput::into_dec(&$e)
    };
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_add() {
        let d1 = dec!(0.32);
        let d2 = dec!(3);
        let d3 = dec!(-1);
        let d4 = d1 * d2 * d2 - d3 - d3 * d3 * d3;
        println!("{}", d4);
    }
}
