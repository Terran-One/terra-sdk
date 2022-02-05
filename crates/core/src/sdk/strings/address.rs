use std::convert::{From, Into};
use std::fmt::Display;
use std::str::FromStr;

use crate::{AccPubKey, ValPubKey};

pub const ADDR_LEN: usize = 32;
pub const ACC_ADDRESS_HRP: &str = "terra";
pub const VAL_ADDRESS_HRP: &str = "terravaloper";
pub const VALCONS_ADDRESS_HRP: &str = "terravalcons";
pub const ACC_PUBKEY_HRP: &str = "terrapub";
pub const VAL_PUBKEY_HRP: &str = "terravaloperpub";
pub const VALCONS_PUBKEY_HRP: &str = "terravalconspub";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AccAddress(pub String);
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ValAddress(pub String);
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ValConsAddress(pub String);

impl AccAddress {
    pub fn unchecked(address: impl Into<String>) -> Self {
        AccAddress(address.into())
    }

    pub fn new(address: impl Into<String>) -> Result<Self, String> {
        let address = address.into();
        if Self::validate(&address) {
            Ok(AccAddress(address))
        } else {
            Err(format!("Invalid account address: {}", address))
        }
    }

    pub fn validate(test: impl Into<String>) -> bool {
        let bech32_str = test.into();
        let parts = bech32::decode(bech32_str.as_str());
        match parts {
            Ok((hrp, data, ..)) => {
                if hrp != ACC_ADDRESS_HRP {
                    return false;
                }
                if data.len() != ADDR_LEN {
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

    pub fn to_val_address(self) -> ValAddress {
        self.into()
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<ValAddress> for AccAddress {
    fn from(val_address: ValAddress) -> Self {
        let (_hrp, data, variant) = bech32::decode(val_address.0.as_str()).unwrap();
        let acc_address = bech32::encode(ACC_ADDRESS_HRP, &data, variant).unwrap();
        AccAddress(acc_address)
    }
}

impl From<AccPubKey> for AccAddress {
    fn from(acc_pubkey: AccPubKey) -> Self {
        let (_hrp, data, variant) = bech32::decode(acc_pubkey.0.as_str()).unwrap();
        let acc_address = bech32::encode(ACC_ADDRESS_HRP, &data, variant).unwrap();
        AccAddress(acc_address)
    }
}

impl FromStr for AccAddress {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        AccAddress::new(s)
    }
}

impl Into<String> for AccAddress {
    fn into(self) -> String {
        self.0
    }
}

impl Display for AccAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ValAddress {
    pub fn unchecked(address: impl Into<String>) -> Self {
        ValAddress(address.into())
    }

    pub fn new(address: impl Into<String>) -> Result<Self, String> {
        let address = address.into();
        if Self::validate(&address) {
            Ok(ValAddress(address))
        } else {
            Err(format!("Invalid validator address: {}", address))
        }
    }

    pub fn validate(test: impl Into<String>) -> bool {
        let bech32_str = test.into();
        let parts = bech32::decode(bech32_str.as_str());
        match parts {
            Ok((hrp, data, ..)) => {
                if hrp != VAL_ADDRESS_HRP {
                    return false;
                }
                if data.len() != ADDR_LEN {
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

    pub fn to_acc_address(self) -> AccAddress {
        self.into()
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<AccAddress> for ValAddress {
    fn from(acc_address: AccAddress) -> Self {
        let (_hrp, data, variant) = bech32::decode(acc_address.0.as_str()).unwrap();
        let val_address = bech32::encode(VAL_ADDRESS_HRP, &data, variant).unwrap();
        ValAddress(val_address)
    }
}

impl From<ValPubKey> for ValAddress {
    fn from(val_pubkey: ValPubKey) -> Self {
        let (_hrp, data, variant) = bech32::decode(val_pubkey.0.as_str()).unwrap();
        let val_address = bech32::encode(VAL_ADDRESS_HRP, &data, variant).unwrap();
        ValAddress(val_address)
    }
}

impl Into<String> for ValAddress {
    fn into(self) -> String {
        self.0
    }
}

impl FromStr for ValAddress {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ValAddress::new(s)
    }
}

impl Display for ValAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl ValConsAddress {
    pub fn unchecked(address: impl Into<String>) -> Self {
        ValConsAddress(address.into())
    }

    pub fn new(address: impl Into<String>) -> Result<Self, String> {
        let address = address.into();
        if Self::validate(&address) {
            Ok(ValConsAddress(address))
        } else {
            Err(format!("Invalid validator consensus address: {}", address))
        }
    }

    pub fn validate(test: impl Into<String>) -> bool {
        let bech32_str = test.into();
        let parts = bech32::decode(bech32_str.as_str());
        match parts {
            Ok((hrp, data, ..)) => {
                if hrp != VALCONS_ADDRESS_HRP {
                    return false;
                }
                if data.len() != ADDR_LEN {
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

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl FromStr for ValConsAddress {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ValConsAddress::new(s)
    }
}

impl Into<String> for ValConsAddress {
    fn into(self) -> String {
        self.0
    }
}

impl Display for ValConsAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_validates_acc_address() {
        let a = AccAddress::validate("terravaloper1pdx498r0hrc2fj36sjhs8vuhrz9hd2cw0yhqtk");
        assert_eq!(a, false);

        let b = AccAddress::validate("terra1pdx498r0h7c2fj36sjhs8vu8rz9hd2cw0tmam9");
        assert_eq!(b, false);

        let c = AccAddress::validate("cosmos176m2p8l3fps3dal7h8gf9jvrv98tu3rqfdht86");
        assert_eq!(c, false);

        let d = AccAddress::validate("terra1pdx498r0hrc2fj36sjhs8vuhrz9hd2cw0tmam9");
        assert_eq!(d, true);

        let e = AccAddress::new("terra1pdx498r0hrc2fj36sjhs8vuhrz9hd2cw0tmam9").is_ok();
        assert_eq!(e, true);
    }

    #[test]
    fn it_validates_val_address() {
        let a = ValAddress::validate("terravaloper1pdx498r0hrc2fj36sjhs8vuhrz9hd2cw0yhqtk");
        assert_eq!(a, true);

        let b = ValAddress::new("terravaloper1pdx498r0hrc2fj36sjhs8vuhrz9hd2cw0yhqtk").is_ok();
        assert_eq!(b, true);
    }

    #[test]
    fn it_converts_from_acc_address() {
        let a = AccAddress::new("terra1pdx498r0hrc2fj36sjhs8vuhrz9hd2cw0tmam9").unwrap();
        let b = a.clone().to_val_address();
        assert_eq!(b.0, "terravaloper1pdx498r0hrc2fj36sjhs8vuhrz9hd2cw0yhqtk");

        let c = ValAddress::from(a.clone());
        assert_eq!(c.0, "terravaloper1pdx498r0hrc2fj36sjhs8vuhrz9hd2cw0yhqtk");
    }

    #[test]
    fn it_converts_from_val_address() {
        let a = ValAddress::new("terravaloper1pdx498r0hrc2fj36sjhs8vuhrz9hd2cw0yhqtk").unwrap();
        let b = a.clone().to_acc_address();
        assert_eq!(b.0, "terra1pdx498r0hrc2fj36sjhs8vuhrz9hd2cw0tmam9");

        let c = AccAddress::from(a.clone());
        assert_eq!(c.0, "terra1pdx498r0hrc2fj36sjhs8vuhrz9hd2cw0tmam9");
    }

    #[test]
    fn it_validates_val_cons_address() {
        let a = ValConsAddress::validate("terravalcons1relcztayk87c3r529rqf3fwdmn8hr6rhcgyrxd");
        assert_eq!(a, true);
    }
}
