use std::convert::{From, Into};
use std::fmt::Display;
use std::str::FromStr;

use crate::{AccAddress, ValAddress, ValConsAddress};

pub const ACC_PUBKEY_HRP: &str = "terrapub";
pub const VAL_PUBKEY_HRP: &str = "terravaloperpub";
pub const VALCONS_PUBKEY_HRP: &str = "terravalconspub";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AccPubKey(pub String);
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ValPubKey(pub String);
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ValConsPubKey(pub String);

impl AccPubKey {
    pub fn unchecked(pubkey: impl Into<String>) -> Self {
        AccPubKey(pubkey.into())
    }

    pub fn new(pubkey: impl Into<String>) -> Result<Self, String> {
        let pubkey = pubkey.into();
        if Self::validate(&pubkey) {
            Ok(AccPubKey(pubkey))
        } else {
            Err(format!("Invalid account public key: {}", pubkey))
        }
    }

    pub fn validate(test: impl Into<String>) -> bool {
        let bech32_str = test.into();
        let parts = bech32::decode(bech32_str.as_str());
        match parts {
            Ok((hrp, data, ..)) => {
                if hrp != ACC_PUBKEY_HRP {
                    return false;
                }
                true
            }
            Err(_) => false,
        }
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn is_valid(&self) -> bool {
        Self::validate(self.0.clone())
    }
}

impl FromStr for AccPubKey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pubkey = s.to_string();
        if Self::validate(&pubkey) {
            Ok(AccPubKey(pubkey))
        } else {
            Err(format!("Invalid account public key: {}", pubkey))
        }
    }
}

impl Into<String> for AccPubKey {
    fn into(self) -> String {
        self.0
    }
}

impl Display for AccPubKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<AccAddress> for AccPubKey {
    fn from(acc_address: AccAddress) -> Self {
        let (_hrp, data, variant) = bech32::decode(acc_address.as_str()).unwrap();
        let bech32_str = bech32::encode(ACC_PUBKEY_HRP, data, variant).unwrap();
        AccPubKey(bech32_str)
    }
}

impl From<ValPubKey> for AccPubKey {
    fn from(val_pubkey: ValPubKey) -> Self {
        let (_hrp, data, variant) = bech32::decode(val_pubkey.as_str()).unwrap();
        let bech32_str = bech32::encode(ACC_PUBKEY_HRP, data, variant).unwrap();
        AccPubKey(bech32_str)
    }
}

impl ValPubKey {
    pub fn unchecked(pubkey: impl Into<String>) -> Self {
        ValPubKey(pubkey.into())
    }

    pub fn new(pubkey: impl Into<String>) -> Result<Self, String> {
        let pubkey = pubkey.into();
        if Self::validate(&pubkey) {
            Ok(ValPubKey(pubkey))
        } else {
            Err(format!("Invalid validator public key: {}", pubkey))
        }
    }

    pub fn validate(test: impl Into<String>) -> bool {
        let bech32_str = test.into();
        let parts = bech32::decode(bech32_str.as_str());
        match parts {
            Ok((hrp, _data, ..)) => {
                if hrp != VAL_PUBKEY_HRP {
                    return false;
                }
                true
            }
            Err(_) => false,
        }
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn is_valid(&self) -> bool {
        Self::validate(self.0.clone())
    }
}

impl From<ValAddress> for ValPubKey {
    fn from(val_address: ValAddress) -> Self {
        let (_hrp, data, variant) = bech32::decode(val_address.as_str()).unwrap();
        let bech32_str = bech32::encode(VAL_PUBKEY_HRP, data, variant).unwrap();
        ValPubKey(bech32_str)
    }
}

impl From<AccPubKey> for ValPubKey {
    fn from(acc_pubkey: AccPubKey) -> Self {
        let (_hrp, data, variant) = bech32::decode(acc_pubkey.as_str()).unwrap();
        let bech32_str = bech32::encode(VAL_PUBKEY_HRP, data, variant).unwrap();
        ValPubKey(bech32_str)
    }
}

impl FromStr for ValPubKey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pubkey = s.to_string();
        if Self::validate(&pubkey) {
            Ok(ValPubKey(pubkey))
        } else {
            Err(format!("Invalid validator public key: {}", pubkey))
        }
    }
}

impl Into<String> for ValPubKey {
    fn into(self) -> String {
        self.0
    }
}

impl Display for ValPubKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ValConsPubKey {
    pub fn unchecked(pubkey: impl Into<String>) -> Self {
        ValConsPubKey(pubkey.into())
    }

    pub fn new(pubkey: impl Into<String>) -> Result<Self, String> {
        let pubkey = pubkey.into();
        if Self::validate(&pubkey) {
            Ok(ValConsPubKey(pubkey))
        } else {
            Err(format!(
                "Invalid validator consensus public key: {}",
                pubkey
            ))
        }
    }

    pub fn validate(test: impl Into<String>) -> bool {
        let bech32_str = test.into();
        let parts = bech32::decode(bech32_str.as_str());
        match parts {
            Ok((hrp, _data, ..)) => {
                if hrp != VALCONS_PUBKEY_HRP {
                    return false;
                }
                true
            }
            Err(_) => false,
        }
    }

    pub fn is_valid(&self) -> bool {
        Self::validate(self.0.clone())
    }
}

impl FromStr for ValConsPubKey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pubkey = s.to_string();
        if Self::validate(&pubkey) {
            Ok(ValConsPubKey(pubkey))
        } else {
            Err(format!(
                "Invalid validator consensus public key: {}",
                pubkey
            ))
        }
    }
}

impl Into<String> for ValConsPubKey {
    fn into(self) -> String {
        self.0
    }
}

impl Display for ValConsPubKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<ValConsAddress> for ValConsPubKey {
    fn from(val_cons_address: ValConsAddress) -> Self {
        let (hrp, data, variant) = bech32::decode(val_cons_address.as_str()).unwrap();
        let bech32_str = bech32::encode(VALCONS_PUBKEY_HRP, data, variant).unwrap();
        ValConsPubKey(bech32_str)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_validates_acc_pubkey() {
        let a = AccPubKey::validate(
            "terravaloperpub1addwnpepqt8ha594svjn3nvfk4ggfn5n8xd3sm3cz6ztxyugwcuqzsuuhhfq5y7accr",
        );
        assert_eq!(a, false);

        let b = AccPubKey::validate("terrapub1x46rqay4d3cssq8gxxvqz8xt6nwlz4tdh39t77");
        assert_eq!(b, true);

        let c = AccPubKey::new("terrapub1x46rqay4d3cssq8gxxvqz8xt6nwlz4tdh39t77").is_ok();
        assert_eq!(c, true);
    }

    #[test]
    fn it_validates_val_pubkey() {
        let a = ValPubKey::validate("terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q");
        assert_eq!(a, false);

        let b = ValPubKey::validate("terravaloperpub12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69gvd5ag");
        assert_eq!(b, true);

        let c = ValPubKey::new("terravaloperpub12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69gvd5ag").is_ok();
        assert_eq!(c, true);
    }

    #[test]
    fn it_validates_valcons_pubkey() {
        let a = ValConsPubKey::validate("terravalcons1relcztayk87c3r529rqf3fwdmn8hr6rhcgyrxd");
        assert_eq!(a, false);

        let b = ValConsPubKey::validate("terravalcons1relcztayk87c3r529rqf3fwdmn8hr6rhcgyrxd");
        assert_eq!(b, true);

        let c = ValConsPubKey::new("terravalcons1relcztayk87c3r529rqf3fwdmn8hr6rhcgyrxd").is_ok();
        assert_eq!(c, true);
    }
}
