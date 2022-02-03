mod api;
use api::*;

use std::cell::RefCell;
use std::sync::Arc;

use self::api::wasm::WasmApi;
use reqwest::Client;

pub struct LCDClient<T>
where
    T: ApiRequester,
{
    requester: Arc<RefCell<T>>,
    pub wasm: wasm::WasmApi<T>,
}

impl LCDClient<BasicApiRequester> {
    pub fn new() -> Self {
        let requester = BasicApiRequester::new(Client::new());
        let arc_req = Arc::new(RefCell::new(requester));
        let weak_req = Arc::downgrade(&arc_req);

        Self {
            requester: arc_req,
            wasm: WasmApi::new(Handle::new(Some(weak_req.clone()))),
        }
    }
}
