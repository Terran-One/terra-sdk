pub mod auth;
pub mod authz;
pub mod bank;
pub mod distribution;
pub mod feegrant;
pub mod gov;
pub mod ibc;
pub mod ibc_transfer;
pub mod market;
pub mod oracle;
pub mod params;
pub mod sdk;
pub mod slashing;
pub mod staking;
pub mod treasury;
pub mod wasm;

pub use sdk::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
