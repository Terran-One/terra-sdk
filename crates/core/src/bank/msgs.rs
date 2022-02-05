use crate::internal::*;
use crate::{AccAddress, Coins};
use serde_json::json;
pub struct MsgSend {
    pub from_address: AccAddress,
    pub to_address: AccAddress,
    pub amount: Coins,
}

// pub struct MsgMultiSend {
//     pub inputs: MsgMultiSendInput,
//     pub outputs: MsgMultiSendOutput,
// }
