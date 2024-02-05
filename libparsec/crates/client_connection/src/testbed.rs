// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{
    any::Any,
    fmt::Debug,
    future::Future,
    path::Path,
    pin::Pin,
    sync::{Arc, Mutex},
};

pub use bytes::Bytes;
pub use reqwest::{header::HeaderMap, Error as RequestError, StatusCode};
use reqwest::{RequestBuilder, Response};

use libparsec_protocol::API_LATEST_VERSION;
use libparsec_testbed::{test_get_testbed_component_store, TestbedEnv, TestbedKind};
use libparsec_types::prelude::*;

use crate::ConnectionResult;

const API_LATEST_MAJOR_VERSION: u32 = API_LATEST_VERSION.version;
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
// In Rust this is incredibly cumbersome to do:
// - `*SendHookFn` can have multiple implementation, so it must be boxed
// - async function is considered as a regular function that return a future...
// - ...future which must be wrapped in a pinned box given it can have multiple
//   implementations
// - `*SendHookFn` is going to be stored in structure that use `Debug`, so
//   it must implement those traits itself.
//   Due to Rust's orphan rule, the only way to do that is to introduce our own trait
//   `*SendHookFnT` on which we implement `Debug`.
//   From that, `SendHookFnT` must be auto-implemented on each closure that respect
//   the send hook signature.

trait LowLevelSendHookFnT:
    Fn(RequestBuilder) -> Pin<Box<dyn Future<Output = Result<ResponseMock, RequestError>> + Send>>
{
}
impl<F> LowLevelSendHookFnT for F where
    F: Fn(
        RequestBuilder,
    ) -> Pin<Box<dyn Future<Output = Result<ResponseMock, RequestError>> + Send>>
{
}
type LowLevelSendHookFn = Box<dyn LowLevelSendHookFnT + Send + Sync>;
impl std::fmt::Debug for dyn LowLevelSendHookFnT + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LowLevelSendHookFn")
    }
}

enum HighLevelSendHookFnCallResult {
    Ok(Pin<Box<dyn Future<Output = Box<dyn Any>> + Send>>),
    TypeCastError(Box<dyn Any>),
}
trait HighLevelSendHookFnT: FnOnce(Box<dyn Any>) -> HighLevelSendHookFnCallResult {}
impl<F> HighLevelSendHookFnT for F where F: FnOnce(Box<dyn Any>) -> HighLevelSendHookFnCallResult {}
type HighLevelSendHookFn = Box<dyn HighLevelSendHookFnT + Send + Sync>;
impl std::fmt::Debug for dyn HighLevelSendHookFnT + Send + Sync {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "HighLevelSendHookFn")
    }
}

#[derive(Debug)]
struct ComponentStore {
    kind: TestbedKind,
    strategy: Mutex<SendHookStrategy>,
}

macro_rules! store_factory {
    ($config_dir: expr) => {
        |env: &TestbedEnv| -> Arc<dyn Any + Send + Sync> {
            Arc::new(ComponentStore {
                kind: env.kind,
                strategy: Mutex::new(SendHookStrategy::Default),
            })
        }
    };
}

#[derive(Debug)]
enum SendHookStrategy {
    Default,
    LowLevelOnce(LowLevelSendHookFn),
    LowLevelMultiple(LowLevelSendHookFn),
    HighLevelOnce(HighLevelSendHookFn),
}

pub(crate) enum HighLevelSendResult<T>
where
    T: ProtocolRequest<API_LATEST_MAJOR_VERSION>,
{
    Resolved(ConnectionResult<<T>::Response>),
    PassToLowLevel(T),
}

#[derive(Debug)]
pub(crate) struct SendHookConfig(Arc<ComponentStore>);

impl Drop for SendHookConfig {
    fn drop(&mut self) {
        let guard = self.0.strategy.lock().expect("Mutex is poisoned");
        if matches!(
            &*guard,
            SendHookStrategy::LowLevelOnce(_) | SendHookStrategy::HighLevelOnce(_)
        ) {
            panic!(
                "Client connection is being destroyed while configured with a mock \
                 expected to be trigger once !"
            );
        }
    }
}

impl SendHookConfig {
    pub async fn high_level_send<T>(&self, request: T) -> HighLevelSendResult<T>
    where
        T: ProtocolRequest<API_LATEST_MAJOR_VERSION> + Debug + 'static,
    {
        // Given mutex is synchronous, we must release it before any await
        let custom_hook_future = {
            let mut guard = self.0.strategy.lock().expect("Mutex is poisoned");
            if let SendHookStrategy::HighLevelOnce(_) = &*guard {
                let custom_hook = {
                    match std::mem::replace(&mut *guard, SendHookStrategy::Default) {
                        SendHookStrategy::HighLevelOnce(custom_hook) => custom_hook,
                        _ => unreachable!(),
                    }
                };

                match custom_hook(Box::new(request)) {
                    HighLevelSendHookFnCallResult::Ok(custom_hook_future) => custom_hook_future,
                    // The request type wasn't what the hook expected
                    HighLevelSendHookFnCallResult::TypeCastError(request) => {
                        let request: Box<T> = request.downcast().expect("Type is known");
                        panic!("Hook got an unexpected request type: {:?}", request)
                    }
                }
            } else {
                return HighLevelSendResult::PassToLowLevel(request);
            }
        };
        let rep_as_any = custom_hook_future.await;
        // Last step: convert back runtime generic type to compile-time specific type
        // In theory this should never fail given if the test code was written with the
        // wrong type the code would have already panicked when we converted the request
        let rep: Box<T::Response> = rep_as_any
            .downcast()
            .expect("Wrong type returned for response !");
        HighLevelSendResult::Resolved(Ok(*rep))
    }

    pub async fn low_level_send(
        &self,
        request_builder: RequestBuilder,
    ) -> Result<ResponseMock, RequestError> {
        // Given mutex is synchronous, we must release it before any await
        let (maybe_custom_hook_future, maybe_request_builder) = {
            let mut guard = self.0.strategy.lock().expect("Mutex is poisoned");
            let strategy = std::mem::replace(&mut *guard, SendHookStrategy::Default);
            match strategy {
                SendHookStrategy::Default => (None, Some(request_builder)),
                SendHookStrategy::LowLevelOnce(custom_hook) => {
                    (Some(custom_hook(request_builder)), None)
                }
                SendHookStrategy::LowLevelMultiple(custom_hook) => {
                    let res = (Some(custom_hook(request_builder)), None);
                    *guard = SendHookStrategy::LowLevelMultiple(custom_hook);
                    res
                }
                SendHookStrategy::HighLevelOnce(_) => {
                    panic!("High level send hook have been triggered !")
                }
            }
        };
        match (self.0.kind, maybe_custom_hook_future, maybe_request_builder) {
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

pub(crate) fn get_send_hook(config_dir: &Path) -> SendHookConfig {
    let store_factory = store_factory!(config_dir);
    let store = test_get_testbed_component_store::<ComponentStore>(
        config_dir,
        STORE_ENTRY_KEY,
        store_factory,
    )
    .unwrap_or_else(|| {
        // Config dir doesn't correspond to a testbed env, so we provide a generic
        // send hook config that never do any mocking
        static NO_TESTBED_ENV_SEND_HOOK: std::sync::OnceLock<Arc<ComponentStore>> =
            std::sync::OnceLock::new();
        NO_TESTBED_ENV_SEND_HOOK
            .get_or_init(|| {
                Arc::new(ComponentStore {
                    kind: TestbedKind::ClientServer,
                    strategy: Mutex::new(SendHookStrategy::Default),
                })
            })
            .to_owned()
    });
    SendHookConfig(store)
}

pub fn test_register_low_level_send_hook<T>(config_dir: &Path, hook: fn(RequestBuilder) -> T)
where
    T: Future<Output = Result<ResponseMock, RequestError>> + Send + 'static,
{
    let store_factory = store_factory!(config_dir);
    test_get_testbed_component_store::<ComponentStore>(config_dir, STORE_ENTRY_KEY, store_factory)
        .map(|store| {
            let mut guard = store.strategy.lock().expect("Mutex is poisoned");
            let cb = move |rb| {
                let future: Pin<Box<dyn Future<Output = _> + Send>> = Box::pin(hook(rb));
                future
            };
            let boxed: Box<dyn LowLevelSendHookFnT + Send + Sync> = Box::new(cb);
            *guard = SendHookStrategy::LowLevelOnce(boxed);
        })
        .expect("Config dir doesn't correspond to a testbed env");
}

pub fn test_register_low_level_send_hook_multicall<T>(
    config_dir: &Path,
    hook: fn(RequestBuilder) -> T,
) where
    T: Future<Output = Result<ResponseMock, RequestError>> + Send + 'static,
{
    let store_factory = store_factory!(config_dir);
    test_get_testbed_component_store::<ComponentStore>(config_dir, STORE_ENTRY_KEY, store_factory)
        .map(|store| {
            let mut guard = store.strategy.lock().expect("Mutex is poisoned");
            let cb = move |rb| {
                let future: Pin<Box<dyn Future<Output = _> + Send>> = Box::pin(hook(rb));
                future
            };
            let boxed: Box<dyn LowLevelSendHookFnT + Send + Sync> = Box::new(cb);
            *guard = SendHookStrategy::LowLevelMultiple(boxed);
        })
        .expect("Config dir doesn't correspond to a testbed env");
}

pub fn test_register_low_level_send_hook_default(config_dir: &Path) {
    let store_factory = store_factory!(config_dir);
    test_get_testbed_component_store::<ComponentStore>(config_dir, STORE_ENTRY_KEY, store_factory)
        .map(|store| {
            let mut guard = store.strategy.lock().expect("Mutex is poisoned");
            *guard = SendHookStrategy::Default;
        })
        .expect("Config dir doesn't correspond to a testbed env");
}

#[macro_export]
macro_rules! test_register_sequence_of_send_hooks {
    ($config_dir_ref: expr, $hook1: expr $(,)?) => {
        let config_dir_ref = $config_dir_ref.to_owned();
        let hook1 = $hook1;
        libparsec_client_connection::test_register_send_hook(&config_dir_ref, move |req| {
            let rep = hook1(req);
            rep
        });
    };

    // We cannot use `$($hook: expr)*` here given we need to first do `let hookX = $hookX;`
    // (i.e. we need to enumerate the hooks and give a unique variable name for each of them)
    // And we need this initial $hook expansion to avoid conflict with the `move` in the
    // nested closures.
    ($config_dir_ref: expr, $hook1: expr, $hook2: expr $(,)?) => {
        let config_dir_ref1 = $config_dir_ref.to_owned();
        let config_dir_ref2 = config_dir_ref1.clone();
        let hook1 = $hook1;
        let hook2 = $hook2;
        libparsec_client_connection::test_register_send_hook(&config_dir_ref1, move |req| {
            let rep = hook1(req);
            libparsec_client_connection::test_register_send_hook(&config_dir_ref2, move |req| {
                let rep = hook2(req);
                rep
            });
            rep
        });
    };

    ($config_dir_ref: expr, $hook1: expr, $hook2: expr, $hook3: expr $(,)?) => {
        let config_dir_ref1 = $config_dir_ref.to_owned();
        let config_dir_ref2 = config_dir_ref1.clone();
        let config_dir_ref3 = config_dir_ref1.clone();
        let hook1 = $hook1;
        let hook2 = $hook2;
        let hook3 = $hook3;
        libparsec_client_connection::test_register_send_hook(&config_dir_ref1, move |req| {
            let rep = hook1(req);
            libparsec_client_connection::test_register_send_hook(&config_dir_ref2, move |req| {
                let rep = hook2(req);
                libparsec_client_connection::test_register_send_hook(
                    &config_dir_ref3,
                    move |req| {
                        let rep = hook3(req);
                        rep
                    },
                );
                rep
            });
            rep
        });
    };

    ($config_dir_ref: expr, $hook1: expr, $hook2: expr, $hook3: expr, $hook4: expr $(,)?) => {
        let config_dir_ref1 = $config_dir_ref.to_owned();
        let config_dir_ref2 = config_dir_ref1.clone();
        let config_dir_ref3 = config_dir_ref1.clone();
        let config_dir_ref4 = config_dir_ref1.clone();
        let hook1 = $hook1;
        let hook2 = $hook2;
        let hook3 = $hook3;
        let hook4 = $hook4;
        libparsec_client_connection::test_register_send_hook(&config_dir_ref1, move |req| {
            let rep = hook1(req);
            libparsec_client_connection::test_register_send_hook(&config_dir_ref2, move |req| {
                let rep = hook2(req);
                libparsec_client_connection::test_register_send_hook(
                    &config_dir_ref3,
                    move |req| {
                        let rep = hook3(req);
                        libparsec_client_connection::test_register_send_hook(
                            &config_dir_ref4,
                            move |req| {
                                let rep = hook4(req);
                                rep
                            },
                        );
                        rep
                    },
                );
                rep
            });
            rep
        });
    };

    ($config_dir_ref: expr, $hook1: expr, $hook2: expr, $hook3: expr, $hook4: expr, $hook5: expr $(,)?) => {
        let config_dir_ref1 = $config_dir_ref.to_owned();
        let config_dir_ref2 = config_dir_ref1.clone();
        let config_dir_ref3 = config_dir_ref1.clone();
        let config_dir_ref4 = config_dir_ref1.clone();
        let config_dir_ref5 = config_dir_ref1.clone();
        let hook1 = $hook1;
        let hook2 = $hook2;
        let hook3 = $hook3;
        let hook4 = $hook4;
        let hook5 = $hook5;
        libparsec_client_connection::test_register_send_hook(&config_dir_ref1, move |req| {
            let rep = hook1(req);
            libparsec_client_connection::test_register_send_hook(&config_dir_ref2, move |req| {
                let rep = hook2(req);
                libparsec_client_connection::test_register_send_hook(
                    &config_dir_ref3,
                    move |req| {
                        let rep = hook3(req);
                        libparsec_client_connection::test_register_send_hook(
                            &config_dir_ref4,
                            move |req| {
                                let rep = hook4(req);
                                libparsec_client_connection::test_register_send_hook(
                                    &config_dir_ref5,
                                    move |req| {
                                        let rep = hook5(req);
                                        rep
                                    },
                                );
                                rep
                            },
                        );
                        rep
                    },
                );
                rep
            });
            rep
        });
    };
}

pub fn test_register_send_hook<F, A>(config_dir: &Path, hook: F)
where
    F: FnOnce(A) -> A::Response + Send + Sync + 'static,
    A: ProtocolRequest<API_LATEST_MAJOR_VERSION> + Send + 'static,
{
    test_register_send_hook_async(config_dir, move |req: A| async move { hook(req) })
}

pub fn test_register_send_hook_async<A, R>(
    config_dir: &Path,
    hook: impl FnOnce(A) -> R + Send + Sync + 'static,
) where
    A: ProtocolRequest<API_LATEST_MAJOR_VERSION> + Send + 'static,
    R: std::future::Future<Output = A::Response> + Send + 'static,
{
    let store_factory = store_factory!(config_dir);
    test_get_testbed_component_store::<ComponentStore>(config_dir, STORE_ENTRY_KEY, store_factory)
        .map(|store| {
            let mut guard = store.strategy.lock().expect("Mutex is poisoned");

            // So the caller provided us with `hook` which is compile-time polymorphic, here
            // we are going to wrap it into a runtime generic callback.
            // The magic sauce is our caller sends (and expects in return) specific types defined
            // at compile-time, so we convert them back and forth with `Any` to be able to turn
            // them into runtime generic types.
            // Of course the drawback is this can blow up at runtime if the test code has
            // specified the wrong type in the callback passed to `test_register_send_hook` ¯\_(ツ)_/¯
            let cb = move |req_as_any: Box<dyn Any>| {
                let req: Box<A> = match req_as_any.downcast() {
                    Ok(req) => req,
                    // Cast has failed, we don't panic here given we have no information
                    // about the type
                    Err(req_as_any) => {
                        return HighLevelSendHookFnCallResult::TypeCastError(req_as_any)
                    }
                };
                let future: Pin<Box<dyn Future<Output = _> + Send>> = Box::pin(async move {
                    let rep = hook(*req).await;
                    let rep_as_any: Box<dyn Any> = Box::new(rep);
                    rep_as_any
                });
                HighLevelSendHookFnCallResult::Ok(future)
            };
            let boxed: Box<dyn HighLevelSendHookFnT + Send + Sync> = Box::new(cb);

            *guard = SendHookStrategy::HighLevelOnce(boxed);
        })
        .expect("Config dir doesn't correspond to a testbed env");
}
