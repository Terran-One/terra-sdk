pub(crate) mod wasm;

use async_trait::async_trait;
use reqwest::{
    header::{HeaderMap, HeaderValue, CONTENT_TYPE},
    Client, Request,
};
use serde::de::DeserializeOwned;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::sync::{Arc, Weak};
pub struct Handle<T> {
    inner: Option<Weak<RefCell<T>>>,
}

impl<T> Handle<T> {
    pub fn new(inner: Option<Weak<RefCell<T>>>) -> Self {
        Self { inner }
    }

    pub fn hold(&self) -> Result<Arc<RefCell<T>>, String> {
        if self.inner.is_none() {
            return Err("unable to hold: handle is None".into());
        }

        match self.inner.as_ref().unwrap().upgrade() {
            Some(inner) => Ok(inner),
            None => Err("unable to hold: reference dropped".into()),
        }
    }
}

#[async_trait]
pub trait ApiRequester {
    async fn get<T>(&self, endpoint: &str) -> Result<T, ()>
    where
        T: DeserializeOwned;
}

pub struct BasicApiRequester {
    client: Client,
}

#[async_trait]
impl ApiRequester for BasicApiRequester {
    async fn get<T>(&self, endpoint: &str) -> Result<T, ()>
    where
        T: DeserializeOwned,
    {
        let request = self._request_get(&endpoint);
        let result = self
            .client
            .borrow()
            .execute(request)
            .await
            .map_err(|_| ())?;
        result.json().await.map_err(|_| ())
    }
}

impl BasicApiRequester {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    fn _request_get(&self, endpoint: &str) -> Request {
        let mut headers = HeaderMap::new();
        headers.append(CONTENT_TYPE, HeaderValue::from_str("text/json").unwrap());

        self.client
            .borrow()
            .get(format!("https://lcd.terra.dev/{}", endpoint))
            .headers(headers)
            .build()
            .unwrap()
    }
}
