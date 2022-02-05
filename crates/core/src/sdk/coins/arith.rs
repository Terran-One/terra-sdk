use std::{
    convert::Into,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign},
};

use crate::sdk::coins::*;
use crate::sdk::numeric::uint128::Uint128;

impl Add<Coin> for Coin {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        if self.denom != rhs.denom {
            panic!("Cannot add coins with different denoms");
        }
        Coin {
            denom: self.denom.clone(),
            amount: self.amount + rhs.amount,
        }
    }
}

impl<'a> Add<&'a Coin> for Coin {
    type Output = Self;
    fn add(self, rhs: &'a Coin) -> Self {
        self.add(rhs.clone())
    }
}

impl<T: Into<Uint128>> Add<T> for Coin {
    type Output = Self;
    fn add(self, rhs: T) -> Self {
        Coin {
            denom: self.denom.clone(),
            amount: self.amount + rhs.into(),
        }
    }
}

impl AddAssign<Coin> for Coin {
    fn add_assign(&mut self, rhs: Coin) {
        if self.denom != rhs.denom {
            panic!("Cannot add coins with different denoms");
        }
        self.amount = self.amount + rhs.amount;
    }
}

impl<'a> AddAssign<&'a Coin> for Coin {
    fn add_assign(&mut self, rhs: &'a Coin) {
        self.add_assign(rhs.clone())
    }
}

impl<T: Into<Uint128>> AddAssign<T> for Coin {
    fn add_assign(&mut self, rhs: T) {
        self.amount = self.amount + rhs.into();
    }
}

impl Sub<Coin> for Coin {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        if self.denom != rhs.denom {
            panic!("Cannot subtract coins with different denoms");
        }
        Coin {
            denom: self.denom.clone(),
            amount: self.amount - rhs.amount,
        }
    }
}

impl<'a> Sub<&'a Coin> for Coin {
    type Output = Self;
    fn sub(self, rhs: &'a Coin) -> Self {
        self.sub(rhs.clone())
    }
}

impl<T: Into<Uint128>> Sub<T> for Coin {
    type Output = Self;
    fn sub(self, rhs: T) -> Self {
        Coin {
            denom: self.denom.clone(),
            amount: self.amount - rhs.into(),
        }
    }
}

impl SubAssign<Coin> for Coin {
    fn sub_assign(&mut self, rhs: Coin) {
        if self.denom != rhs.denom {
            panic!("Cannot subtract coins with different denoms");
        }
        self.amount = self.amount - rhs.amount;
    }
}

impl<'a> SubAssign<&'a Coin> for Coin {
    fn sub_assign(&mut self, rhs: &'a Coin) {
        self.sub_assign(rhs.clone())
    }
}

impl<T: Into<Uint128>> SubAssign<T> for Coin {
    fn sub_assign(&mut self, rhs: T) {
        self.amount = self.amount - rhs.into();
    }
}

impl<T: Into<Uint128>> Mul<T> for Coin {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Coin {
            denom: self.denom.clone(),
            amount: self.amount * rhs.into(),
        }
    }
}

impl<T: Into<Uint128>> MulAssign<T> for Coin {
    fn mul_assign(&mut self, rhs: T) {
        self.amount = self.amount * rhs.into();
    }
}

impl<T: Into<Uint128>> Div<T> for Coin {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        Coin {
            denom: self.denom.clone(),
            amount: self.amount / rhs.into(),
        }
    }
}

impl<T: Into<Uint128>> DivAssign<T> for Coin {
    fn div_assign(&mut self, rhs: T) {
        self.amount = self.amount / rhs.into();
    }
}

impl<T: Into<Uint128>> Rem<T> for Coin {
    type Output = Self;
    fn rem(self, rhs: T) -> Self {
        Coin {
            denom: self.denom.clone(),
            amount: self.amount % rhs.into(),
        }
    }
}

impl<T: Into<Uint128>> RemAssign<T> for Coin {
    fn rem_assign(&mut self, rhs: T) {
        self.amount = self.amount % rhs.into();
    }
}

impl Add<Coins> for Coins {
    type Output = Self;

    fn add(self, rhs: Coins) -> Self {
        let mut new_coins = self.clone();
        for coin in rhs.iter() {
            if let Some(x) = self.get(&coin.denom) {
                new_coins.set(coin.denom.clone(), x.clone().add(coin.amount));
            } else {
                new_coins.set(coin.denom.clone(), coin.clone());
            }
        }
        new_coins
    }
}

impl<T> Add<T> for Coins
where
    T: Into<Coin>,
{
    type Output = Self;

    fn add(self, rhs: T) -> Self {
        let rhs = rhs.into();
        let mut new_coins = self.clone();
        if let Some(x) = self.get(&rhs.denom) {
            new_coins.set(rhs.denom.clone(), x.clone().add(&rhs));
        } else {
            new_coins.set(rhs.denom.clone(), rhs.clone());
        }
        new_coins
    }
}

impl Sub<Coins> for Coins {
    type Output = Self;

    fn sub(self, rhs: Coins) -> Self {
        let mut new_coins = self.clone();
        for coin in rhs.iter() {
            if let Some(x) = self.get(&coin.denom) {
                new_coins.set(coin.denom.clone(), x.clone().sub(coin.amount));
            } else {
                new_coins.set(coin.denom.clone(), coin.clone());
            }
        }
        new_coins
    }
}

impl AddAssign<Coins> for Coins {
    fn add_assign(&mut self, rhs: Coins) {
        for coin in rhs.iter() {
            if let Some(x) = self.get(&coin.denom) {
                self.set(coin.denom.clone(), x.clone().add(coin.amount));
            } else {
                self.set(coin.denom.clone(), coin.clone());
            }
        }
    }
}

impl<T: Into<Coin>> AddAssign<T> for Coins {
    fn add_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        if let Some(x) = self.get_mut(&rhs.denom) {
            *x += rhs;
        } else {
            self.set(rhs.denom.clone(), rhs);
        }
    }
}

impl SubAssign<Coins> for Coins {
    fn sub_assign(&mut self, rhs: Coins) {
        for coin in rhs.iter() {
            if let Some(x) = self.get_mut(&coin.denom) {
                *x -= coin.amount;
            } else {
                self.set(coin.denom.clone(), coin.clone());
            }
        }
    }
}

impl<T: Into<Coin>> SubAssign<T> for Coins {
    fn sub_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        if let Some(x) = self.get_mut(&rhs.denom) {
            *x -= rhs;
        } else {
            panic!("cannot subtract coin {}", rhs);
        }
    }
}

impl<T: Into<Uint128>> Mul<T> for Coins {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        let rhs = rhs.into();
        let mut new_coins = self.clone();
        for coin in self.iter() {
            new_coins.set(coin.denom.clone(), coin.clone().mul(rhs.clone()));
        }
        new_coins
    }
}

impl<T: Into<Uint128>> MulAssign<T> for Coins {
    fn mul_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        for coin in self.iter_mut() {
            *coin *= rhs;
        }
    }
}

impl<T: Into<Uint128>> Div<T> for Coins {
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        let rhs = rhs.into();
        let mut new_coins = self.clone();
        for coin in self.iter() {
            new_coins.set(coin.denom.clone(), coin.clone().div(rhs.clone()));
        }
        new_coins
    }
}

impl<T: Into<Uint128>> DivAssign<T> for Coins {
    fn div_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        for coin in self.iter_mut() {
            *coin /= rhs;
        }
    }
}

impl<T: Into<Uint128>> Rem<T> for Coins {
    type Output = Self;

    fn rem(self, rhs: T) -> Self {
        let rhs = rhs.into();
        let mut new_coins = self.clone();
        for coin in self.iter() {
            new_coins.set(coin.denom.clone(), coin.clone().rem(rhs.clone()));
        }
        new_coins
    }
}

impl<T: Into<Uint128>> RemAssign<T> for Coins {
    fn rem_assign(&mut self, rhs: T) {
        let rhs = rhs.into();
        for coin in self.iter_mut() {
            *coin %= rhs;
        }
    }
}
