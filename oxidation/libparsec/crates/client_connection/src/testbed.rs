// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{fmt::Debug, future::Future, path::Path, pin::Pin};

use bytes::Bytes;
use reqwest::{header::HeaderMap, Error as RequestError, RequestBuilder, Response, StatusCode};

use libparsec_testbed::{test_get_testbed, TestbedKind};

#[derive(Debug)]
pub(crate) enum ResponseWrapper {
    Mocked((StatusCode, HeaderMap, Bytes)),
    Real(Response),
}

// We want the testbed to return an async function pointer to mock client requests.
// In Rust this is increadibly cumbersome to do:
// - `SendHookFn` can have multiple implementation, so it must be boxed
// - async function are considered as regular function that return a future...
// - ...future which must be wrapped in a pinned box given it can have multiple
//   implementations
// - `SendHookFn` is going to be stored in structure that use `Debug`, so
//   it must implement those traits itself.
//   Due to Rust's orphan rule, the only way to do that is to indroduce our own trait
//   `SendHookFnT` on which we implement `Debug`.
//   From that, `SendHookFnT` must be auto-implemented on each closure that respect
//   the send hook signature.
pub(crate) trait SendHookFnT:
    Fn(RequestBuilder) -> Pin<Box<dyn Future<Output = Result<ResponseWrapper, RequestError>> + Send>>
{
}
impl<F> SendHookFnT for F where
    F: Fn(
        RequestBuilder,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseWrapper, RequestError>> + Send>>
{
}
pub(crate) type SendHookFn = Box<dyn SendHookFnT + Send + Sync>;
impl std::fmt::Debug for dyn SendHookFnT + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SendHookFn")
    }
}

impl ResponseWrapper {
    pub fn status(&self) -> StatusCode {
        match self {
            ResponseWrapper::Mocked((status_code, _, _)) => status_code.to_owned(),
            ResponseWrapper::Real(response) => response.status(),
        }
    }
    pub fn headers(&self) -> &HeaderMap {
        match self {
            ResponseWrapper::Mocked((_, headers, _)) => headers,
            ResponseWrapper::Real(response) => response.headers(),
        }
    }
    pub async fn bytes(self) -> Result<Bytes, RequestError> {
        match self {
            ResponseWrapper::Mocked((_, _, bytes)) => Ok(bytes),
            ResponseWrapper::Real(response) => response.bytes().await,
        }
    }
}

async fn send_hook_always_send_to_server(
    request_builder: RequestBuilder,
) -> Result<ResponseWrapper, RequestError> {
    let response = request_builder.send().await?;
    Ok(ResponseWrapper::Real(response))
}

async fn send_hook_always_server_offline(
    _request_builder: RequestBuilder,
) -> Result<ResponseWrapper, RequestError> {
    Ok(ResponseWrapper::Mocked((
        StatusCode::SERVICE_UNAVAILABLE,
        HeaderMap::new(),
        Bytes::new(),
    )))
}

macro_rules! wrap_send_hook_fn {
    ($cb: ident) => {{
        let cb = |rb| {
            let future: Pin<Box<dyn Future<Output = _> + Send>> = Box::pin($cb(rb));
            future
        };
        let boxed: Box<dyn SendHookFnT + Send + Sync> = Box::new(cb);
        boxed
    }};
}

pub(crate) fn get_send_hook(config_dir: &Path) -> SendHookFn {
    let kind = test_get_testbed(config_dir).map(|env| env.kind);

    match kind {
        // There is no server listening, must prevent the client from actually
        // sending a request !
        Some(TestbedKind::ClientOnly) => wrap_send_hook_fn!(send_hook_always_server_offline),
        // Two possibilities:
        // - Server is running (typically listening on localhost), we can use the
        //   client as-is to access it
        // - We are not in a testbed, then we should not mock. However we cannot just
        //   have this function return a `None` and have the regular code run.
        //   Given `reqwest::Response` cannot be created from scratch, we instead
        //   replace it by `ResponseWrapper` when compiled in with testbed support.
        //   Hence why this function must always return a send hook: it is the only
        //   way for the caller code to get a `ResponseWrapper`.
        Some(TestbedKind::ClientServer) | None => {
            wrap_send_hook_fn!(send_hook_always_send_to_server)
        }
    }
}
