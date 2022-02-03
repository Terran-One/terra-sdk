use crate::lcd_client::api::{ApiRequester, Handle};
use serde_json::Value;

pub struct WasmApi<T>
where
    T: ApiRequester,
{
    requester: Handle<T>,
}

impl<T> WasmApi<T>
where
    T: ApiRequester,
{
    pub fn new(requester: Handle<T>) -> Self {
        Self { requester }
    }

    pub async fn parameters(&self) -> Result<Value, String> {
        let endpoint = "wasm/parameters";
        Ok(self.requester.hold()?.borrow().get(endpoint).await.unwrap())
    }
}
