use serde_json::{json, Value};

pub trait Msg: Sized {
    const TYPE: &'static str;

    fn msg_value(&self) -> Value;

    fn to_json(&self) -> Value {
        json!({
            "type": Self::TYPE,
            "value": self.to_json(),
        })
    }

    fn from_json(json: &Value) -> Self {
        unimplemented!()
    }
}

#[macro_export]
macro_rules! impl_json_ser {
    ($t:ty, $f:expr) => {
        impl ToJson for $t {
            fn to_json(&self) -> Value {
                f(&self)
            }
        }
    };
}

pub trait ToJson {
    fn to_json(&self) -> Value;
}

pub trait FromJson {
    fn from_json(json: &Value) -> Self;
}

pub trait ToAmino {}

pub trait ToProto {}
