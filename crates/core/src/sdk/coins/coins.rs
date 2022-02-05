use crate::coin::*;
use std::cmp::{Eq, PartialEq};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Coins(HashMap<String, Coin>);

impl Coins {
    pub fn new() -> Self {
        Coins(HashMap::new())
    }

    pub fn add_coin(&mut self, coin: Coin) {
        self.0.insert(coin.denom.clone(), coin);
    }

    pub fn remove_coin(&mut self, coin: Coin) {
        self.0.remove(&coin.denom);
    }

    pub fn get(&self, denom: impl Into<String>) -> Option<&Coin> {
        self.0.get(denom.into().as_str())
    }

    pub fn get_mut(&mut self, denom: impl Into<String>) -> Option<&mut Coin> {
        self.0.get_mut(denom.into().as_str())
    }

    pub fn set(&mut self, denom: impl Into<String>, coin: Coin) {
        self.0.insert(denom.into(), coin.clone());
    }

    pub fn remove(&mut self, denom: &str) {
        self.0.remove(denom);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Coin> {
        self.0.iter().map(|(_, v)| v).into_iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Coin> {
        self.0.iter_mut().map(|(_, v)| v).into_iter()
    }

    pub fn has_denom(&self, denom: &str) -> bool {
        self.0.contains_key(denom)
    }

    pub fn filter(&self, f: impl Fn(&Coin) -> bool) -> Coins {
        let mut coins = Coins::new();
        for coin in self.iter() {
            if f(coin) {
                coins.set(coin.denom.clone(), coin.clone());
            }
        }
        coins
    }

    pub fn parse(s: &str) -> Result<Self, String> {
        let mut coins = Coins::new();
        for coin in s.split(',') {
            let parsed_coin = Coin::parse(coin)?;
            coins.set(parsed_coin.denom.clone(), parsed_coin);
        }
        Ok(coins)
    }
}

impl IntoIterator for Coins {
    type Item = Coin;
    type IntoIter = std::vec::IntoIter<Coin>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
            .into_iter()
            .map(|(_, v)| v)
            .collect::<Vec<_>>()
            .into_iter()
    }
}

impl<T> FromIterator<T> for Coins
where
    T: Into<Coin>,
{
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        let mut coins = Coins::new();
        iter.into_iter().for_each(|coin| {
            let coin = coin.into();
            coins.0.insert(coin.denom.clone(), coin);
        });
        coins
    }
}

impl FromStr for Coins {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Coins::parse(s)
    }
}

impl<T> From<Vec<T>> for Coins
where
    T: Into<Coin>,
{
    fn from(coin_list: Vec<T>) -> Self {
        Self::from_iter(coin_list.into_iter())
    }
}
