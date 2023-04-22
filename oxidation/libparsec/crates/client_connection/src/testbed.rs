// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{
    fmt::Debug,
    future::Future,
    path::Path,
    pin::Pin,
    sync::{Arc, Mutex},
};

pub use bytes::Bytes;
use once_cell::sync::OnceCell;
pub use reqwest::{header::HeaderMap, Error as RequestError, StatusCode};
use reqwest::{RequestBuilder, Response};

use libparsec_testbed::{test_get_testbed, TestbedKind};

const STORE_ENTRY_KEY: &str = "client_connection";

/// Represent a response of an HTTP request sent by the client
/// The trick is this can be either the result of a real HTTP request (e.g. if the
/// current testbed env communicate with a testbed server) or a mocked response.
///
/// This structure is required given `reqwest::Response`'s constructor is private,
/// hence we cannot build an arbitrary `reqwest::Response` when mocking the response.
#[derive(Debug)]
pub enum ResponseMock {
    Mocked((StatusCode, HeaderMap, Bytes)),
    Real(Response),
}

impl ResponseMock {
    pub fn status(&self) -> StatusCode {
        match self {
            ResponseMock::Mocked((status_code, _, _)) => status_code.to_owned(),
            ResponseMock::Real(response) => response.status(),
        }
    }
    pub fn headers(&self) -> &HeaderMap {
        match self {
            ResponseMock::Mocked((_, headers, _)) => headers,
            ResponseMock::Real(response) => response.headers(),
        }
    }
    pub async fn bytes(self) -> Result<Bytes, RequestError> {
        match self {
            ResponseMock::Mocked((_, _, bytes)) => Ok(bytes),
            ResponseMock::Real(response) => response.bytes().await,
        }
    }
}

// We want the testbed to return an async function pointer to mock client requests.
// In Rust this is increadibly cumbersome to do:
// - `SendHookFn` can have multiple implementation, so it must be boxed
// - async function is considered as a regular function that return a future...
// - ...future which must be wrapped in a pinned box given it can have multiple
//   implementations
// - `SendHookFn` is going to be stored in structure that use `Debug`, so
//   it must implement those traits itself.
//   Due to Rust's orphan rule, the only way to do that is to indroduce our own trait
//   `SendHookFnT` on which we implement `Debug`.
//   From that, `SendHookFnT` must be auto-implemented on each closure that respect
//   the send hook signature.
pub(crate) trait SendHookFnT:
    Fn(RequestBuilder) -> Pin<Box<dyn Future<Output = Result<ResponseMock, RequestError>> + Send>>
{
}
impl<F> SendHookFnT for F where
    F: Fn(
        RequestBuilder,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseMock, RequestError>> + Send>>
{
}
pub(crate) type SendHookFn = Box<dyn SendHookFnT + Send + Sync>;
impl std::fmt::Debug for dyn SendHookFnT + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SendHookFn")
    }
}

#[derive(Debug)]
pub(crate) struct SendHookConfig {
    kind: TestbedKind,
    custom_hook: Mutex<Option<SendHookFn>>,
}

impl SendHookConfig {
    fn new(kind: TestbedKind) -> Self {
        Self {
            kind,
            custom_hook: Mutex::new(None),
        }
    }

    pub async fn send(
        &self,
        request_builder: RequestBuilder,
    ) -> Result<ResponseMock, RequestError> {
        // Given custom hook's mutex is synchronous, we must release it before any await
        let (maybe_custom_hook_future, maybe_request_builder) =
            match &*self.custom_hook.lock().expect("Mutex is poisoned") {
                Some(custom_hook) => (Some(custom_hook(request_builder)), None),
                None => (None, Some(request_builder)),
            };
        match (self.kind, maybe_custom_hook_future, maybe_request_builder) {
            // There is no server listening, must prevent the client from actually
            // sending a request !
            (TestbedKind::ClientOnly, None, _) => Ok(ResponseMock::Mocked((
                StatusCode::SERVICE_UNAVAILABLE,
                HeaderMap::new(),
                Bytes::new(),
            ))),
            // Two possibilities:
            // - Server is running (typically listening on localhost), we can use the
            //   client as-is to access it
            // - We are not in a testbed, then we should not mock. However we cannot just
            //   have this function return a `None` and have the regular code run.
            //   Given `reqwest::Response` cannot be created from scratch, we instead
            //   replace it by `ResponseWrapper` when compiled in with testbed support.
            (TestbedKind::ClientServer, None, request_builder) => {
                let request_builder =
                    request_builder.expect("request builder should not have been already used");
                request_builder.send().await.map(ResponseMock::Real)
            }
            // Last case: we have a custom hook... so use it !
            (_, Some(boxed_future), _) => boxed_future.await,
        }
    }
}

/// Retrieve (or create) our send hook config, check it type and pass it to `cb`.
/// Return `None` if `config_dir` doesn't correspond to a testbed env.
fn with_send_hook_config<T>(
    config_dir: &Path,
    cb: impl FnOnce(&mut Arc<SendHookConfig>) -> T,
) -> Option<T> {
    test_get_testbed(config_dir).map(|env| {
        let mut global_store = env.persistence_store.lock().expect("Mutex is poisoned");
        let send_hook = global_store
            .entry(STORE_ENTRY_KEY)
            .or_insert_with(|| Box::new(Arc::new(SendHookConfig::new(env.kind))));
        let send_hook = send_hook
            .downcast_mut::<Arc<SendHookConfig>>()
            .expect("Unexpected persistence store type for client_connection");
        cb(send_hook)
    })
}

pub(crate) fn get_send_hook(config_dir: &Path) -> Arc<SendHookConfig> {
    with_send_hook_config(config_dir, |send_hook| send_hook.to_owned()).unwrap_or_else(|| {
        // Config dir doesn't correspond to a testbed env, so we provide a generic
        // send hook config that never do any mocking
        static NO_TESTBED_ENV_SEND_HOOK: OnceCell<Arc<SendHookConfig>> = OnceCell::new();
        NO_TESTBED_ENV_SEND_HOOK
            .get_or_init(|| Arc::new(SendHookConfig::new(TestbedKind::ClientServer)))
            .clone()
    })
}

macro_rules! wrap_send_hook_fn {
    ($cb: ident) => {{
        let cb = move |rb| {
            let future: Pin<Box<dyn Future<Output = _> + Send>> = Box::pin($cb(rb));
            future
        };
        let boxed: Box<dyn SendHookFnT + Send + Sync> = Box::new(cb);
        boxed
    }};
}

pub fn test_register_send_hook<T>(config_dir: &Path, hook: Option<fn(RequestBuilder) -> T>)
where
    T: Future<Output = Result<ResponseMock, RequestError>> + Send + 'static,
{
    with_send_hook_config(config_dir, move |send_hook_config| {
        let mut guard = send_hook_config
            .custom_hook
            .lock()
            .expect("Mutex is poisoned");
        *guard = hook.map(|cb| wrap_send_hook_fn!(cb));
    })
    .expect("Config dir doesn't correspond to a testbed env");
}
