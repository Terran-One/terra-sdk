use crate::sdk::coin::{Coin, Coins};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

impl<T> Add<T> for Coin
where
    T: Into<Self>,
{
    type Output = Self;
    fn add(self, other: T) -> Self {
        Self {
            denom: self.denom,
            amount: self.amount + other.into().amount,
        }
    }
}

impl<T> AddAssign<T> for Coin
where
    T: Into<Self>,
{
    fn add_assign(&mut self, rhs: T) {
        self.amount = self.amount + rhs.into().amount
    }
}
