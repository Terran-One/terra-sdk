mod api;
use api::*;

use reqwest::header::*;
use reqwest::{Client, Request};
use serde::de::DeserializeOwned;

use std::borrow::Borrow;
use std::cell::RefCell;
use std::sync::Arc;

use async_trait::async_trait;

use self::api::wasm::{Handle, WasmApi};

pub struct LCDClient<T>
where
    T: ApiRequester,
{
    requester: Arc<RefCell<T>>,
    pub wasm: wasm::WasmApi<T>,
}

pub struct BlockingLCDClient<'a> {
    client: Option<&'a Client>,
    wasm: &'a wasm::BlockingWasmApi<'a>,
}

impl LCDClient<BasicApiRequester> {
    pub fn new(requester: BasicApiRequester) -> Self {
        let arc_req = Arc::new(RefCell::new(requester));
        let weak_req = Arc::downgrade(&arc_req);

        Self {
            requester: arc_req,
            wasm: WasmApi::new(Handle::new(Some(weak_req.clone()))),
        }
    }
}
