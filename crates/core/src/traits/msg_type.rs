use serde::{Deserialize, Serialize};
use serde_json::Value;
pub trait JsonSer: Serialize {
    fn to_json(&self) -> Result<Value, String>;
}
pub trait JsonDes<'de>: Deserialize<'de> {
    fn from_json(value: &Value) -> Result<Self, String>;
}
