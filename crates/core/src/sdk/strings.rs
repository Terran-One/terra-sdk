use std::convert::{From, Into};
use std::fmt::Display;
use std::str::FromStr;

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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AccPubKey(pub String);
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ValPubKey(pub String);
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ValConsPubKey(pub String);

macro_rules! impl_bech32_string {
    ($t:ty, $hrp:expr, $desc:expr, $is_addr:expr) => {
        impl $t {
            pub fn unchecked(address: impl Into<String>) -> Self {
                Self(address.into())
            }

            pub fn new(address: impl Into<String>) -> Result<Self, String> {
                let address = address.into();
                if Self::validate(&address) {
                    Ok(Self(address))
                } else {
                    Err(format!("Invalid {}: {}", $desc, address))
                }
            }

            pub fn validate(test: impl Into<String>) -> bool {
                let bech32_str = test.into();
                let parts = bech32::decode(bech32_str.as_str());
                match parts {
                    Ok((hrp, data, ..)) => {
                        if hrp != $hrp {
                            return false;
                        }
                        if $is_addr {
                            if data.len() != ADDR_LEN {
                                return false;
                            }
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

        impl From<$t> for String {
            fn from(address: $t) -> Self {
                address.0
            }
        }

        impl FromStr for $t {
            type Err = String;

            fn from_str(address: &str) -> Result<Self, Self::Err> {
                <$t>::new(address)
            }
        }

        impl Display for $t {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

macro_rules! impl_bech32_convert {
    ($t_from:ty, $t_to:ty, $to_fn:ident, $hrp_to:expr) => {
        impl $t_from {
            pub fn $to_fn(self) -> $t_to {
                <$t_to>::from(self)
            }
        }

        impl From<$t_from> for $t_to {
            fn from(address: $t_from) -> Self {
                let bech32_str = address.0.clone();
                let (_hrp, data, variant) = bech32::decode(bech32_str.as_str()).unwrap();
                let new_str = bech32::encode($hrp_to, data, variant).unwrap();
                Self(new_str)
            }
        }
    };
}

impl_bech32_string!(AccAddress, ACC_ADDRESS_HRP, "account address", true);
impl_bech32_string!(ValAddress, VAL_ADDRESS_HRP, "validator address", true);
impl_bech32_string!(
    ValConsAddress,
    VALCONS_ADDRESS_HRP,
    "validator consensus address",
    true
);
impl_bech32_string!(AccPubKey, ACC_PUBKEY_HRP, "account public key", false);
impl_bech32_string!(ValPubKey, VAL_PUBKEY_HRP, "validator public key", false);
impl_bech32_string!(
    ValConsPubKey,
    VALCONS_PUBKEY_HRP,
    "validator consensus public key",
    false
);
impl_bech32_convert!(AccAddress, ValAddress, to_val_address, VAL_ADDRESS_HRP);
impl_bech32_convert!(ValAddress, AccAddress, to_acc_address, ACC_ADDRESS_HRP);
impl_bech32_convert!(AccAddress, AccPubKey, to_acc_pubkey, ACC_PUBKEY_HRP);
impl_bech32_convert!(ValAddress, ValPubKey, to_val_pubkey, VAL_PUBKEY_HRP);
impl_bech32_convert!(
    ValConsAddress,
    ValConsPubKey,
    to_val_cons_pubkey,
    VALCONS_PUBKEY_HRP
);
impl_bech32_convert!(
    ValConsPubKey,
    ValConsAddress,
    to_val_cons_address,
    VALCONS_ADDRESS_HRP
);

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
