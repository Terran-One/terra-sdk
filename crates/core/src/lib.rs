// pub mod auth;
// pub mod authz;
// pub mod bank;
// pub mod distribution;
// pub mod feegrant;
// pub mod gov;
// pub mod ibc;
// pub mod ibc_transfer;
// pub mod market;
// pub mod oracle;
// pub mod params;
// pub mod sdk;
// pub mod slashing;
// pub mod staking;
// pub mod treasury;
// pub mod wasm;

// pub use sdk::*;

use terra_sdk_internal::Msg;

pub trait Msg {
    fn to_json(&self) -> String;
}

#[derive(Msg)]
#[msg_type("", proto = "")]
pub struct MsgTest {
    pub address: String,
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        let a = MsgTest {
            address: "3".to_string(),
        };

        assert_eq!("hello", a.to_json());
    }
}
