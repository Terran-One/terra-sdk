use crate::{AccAddress, Coins, JsonSer};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct MsgSend {
    pub from_address: AccAddress,
    pub to_address: AccAddress,
    pub amount: Coins,
}

impl MsgSend {
    pub fn new(
        from_address: impl Into<AccAddress>,
        to_address: impl Into<AccAddress>,
        amount: impl Into<Coins>,
    ) -> Self {
        Self {
            from_address: from_address.into(),
            to_address: to_address.into(),
            amount: amount.into(),
        }
    }
}

impl JsonSer for MsgSend {
    fn to_json(&self) -> Result<Value, String> {
        Ok(json!(
            {
                "type": "bank/send",
                "value": serde_json::to_value(self).map_err(|_| "failed to serialize MsgSend")?,
            }
        ))
    }
}

// pub struct MsgMultiSend {
//     pub inputs: MsgMultiSendInput,
//     pub outputs: MsgMultiSendOutput,
// }

#[cfg(test)]
mod tests {

    use super::*;
    use serde::*;
    use serde_json::*;

    const MSG: &MsgSend = &MsgSend {
        from_address: "terra1y4umfuqfg76t8mfcff6zzx7elvy93jtp4xcdvw",
        to_address: "terra1v9ku44wycfnsucez6fp085f5fsksp47u9x8jr4",
        amount: "1000000uluna,2000000uusd",
    };

    #[test]
    fn it_serializes_to_json() {
        let item = MSG.to_json().as_str();
    }
    #[test]
    fn it_serializes() {
        let msg = MsgSend {
            from_address: "terra1y4umfuqfg76t8mfcff6zzx7elvy93jtp4xcdvw".into(),
            to_address: "terra1v9ku44wycfnsucez6fp085f5fsksp47u9x8jr4".into(),
            amount: "1000000uluna,2000000uusd".into(),
        };

        let msg2 = MsgSend::deserialize(json!({
            "from_address": "terra1y4umfuqfg76t8mfcff6zzx7elvy93jtp4xcdvw",
            "to_address": "terra1v9ku44wycfnsucez6fp085f5fsksp47u9x8jr4",
            "amount": [
                {
                    "denom": "uluna",
                    "amount": "1000000"
                },
                {
                    "denom": "uusd",
                    "amount": "2000000"
                }
            ]
        }))
        .unwrap();
        assert_eq!(msg, msg2);
    }
}
