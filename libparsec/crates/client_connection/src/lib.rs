// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![doc = include_str!("../README.md")]

mod anonymous_cmds;
mod anonymous_server_cmds;
mod authenticated_account_cmds;
mod authenticated_cmds;
mod error;
mod invited_cmds;
#[cfg(feature = "test-with-testbed")]
mod testbed;
#[cfg(feature = "test-with-testbed")]
mod testbed_send_hook_helpers;

pub use anonymous_cmds::AnonymousCmds;
pub use anonymous_server_cmds::AnonymousServerCmds;
pub use authenticated_account_cmds::{AccountAuthMethod, AuthenticatedAccountCmds};
pub use authenticated_cmds::{
    sse::{RateLimiter, SSEEvent, SSEResponseOrMissedEvents, SSEStream},
    AuthenticatedCmds, PARSEC_AUTH_METHOD,
};
pub use error::{ConnectionError, ConnectionResult};
pub use invited_cmds::InvitedCmds;
#[cfg(feature = "test-with-testbed")]
// Also re-expose reqwest&bytes stuff to simplify building mock response
pub use testbed::{
    test_register_low_level_send_hook, test_register_low_level_send_hook_default,
    test_register_low_level_send_hook_multicall, test_register_send_hook, Bytes, HeaderMap,
    HeaderName, HeaderValue, ResponseMock, StatusCode,
};
// Re-expose
pub use libparsec_platform_http_proxy::ProxyConfig;
pub use libparsec_protocol as protocol;

/// We send the HTTP request with the body encoded in `msgpack` format.
/// This is the corresponding mime type to convey that info.
pub const PARSEC_CONTENT_TYPE: &str = "application/msgpack";

pub const API_VERSION_HEADER_NAME: &str = "Api-Version";

macro_rules! client_user_agent {
    ($os:literal) => {
        pub(crate) const CLIENT_USER_AGENT: &str = concat!(
            "Parsec-Client/",
            std::include_str!("../../../version"),
            " ",
            $os
        );
    };
}

#[cfg(target_os = "linux")]
client_user_agent!("Linux");
#[cfg(target_os = "windows")]
client_user_agent!("Windows");
#[cfg(target_os = "macos")]
client_user_agent!("MacOS");
#[cfg(target_os = "ios")]
client_user_agent!("iOS");
#[cfg(target_os = "android")]
client_user_agent!("Android");
#[cfg(target_arch = "wasm32")]
client_user_agent!("Web");

pub fn build_client() -> libparsec_types::anyhow::Result<reqwest::Client> {
    build_client_with_proxy(ProxyConfig::default())
}

pub fn build_client_with_proxy(
    proxy: ProxyConfig,
) -> libparsec_types::anyhow::Result<reqwest::Client> {
    let builder = reqwest::ClientBuilder::default().user_agent(CLIENT_USER_AGENT);

    // Configure the minimum accepted TLS version, ideally we would use TLSv1.3 as Parsec is
    // recent and does not need to support legacy configuration.
    // But we are limited by `native-tls` that does not support that version.
    #[cfg(not(target_arch = "wasm32"))]
    let builder = builder.min_tls_version(reqwest::tls::Version::TLS_1_2);

    #[cfg(not(target_arch = "wasm32"))]
    let builder = if let Some(cafile) = std::env::var_os("SSL_CAFILE") {
        use libparsec_types::anyhow::Context;
        log::debug!("Using SSL_CAFILE: {cafile:?}");
        let cafile = std::fs::read(cafile).context("Reading SSL_CAFILE")?;
        let certs = reqwest::Certificate::from_pem_bundle(&cafile)?;
        log::trace!("Collected {} certificates", certs.len());
        certs
            .into_iter()
            .fold(builder, |builder, cert| builder.add_root_certificate(cert))
    } else {
        builder
    };

    let builder = proxy.configure_http_client(builder);

    Ok(builder.build()?)
}

#[cfg(test)]
#[test]
fn user_agent_without_trailing_whitespace() {
    use libparsec_tests_lite::prelude::*;

    // It's easy to slip a \n when modifying the `../../../version` file
    p_assert_eq!(CLIENT_USER_AGENT.trim(), CLIENT_USER_AGENT);
    assert!(!CLIENT_USER_AGENT.contains('\t'), "{CLIENT_USER_AGENT:?}");
    assert!(!CLIENT_USER_AGENT.contains('\n'), "{CLIENT_USER_AGENT:?}");

    // User-Agent should be something like "Parsec-Client/0.0.0 Linux"
    let parts: Vec<_> = CLIENT_USER_AGENT.split_whitespace().collect();
    p_assert_eq!(parts.len(), 2, "{:?}", CLIENT_USER_AGENT);
}

// Testing on web requires this macro configuration to be present anywhere in
// the crate.
// In most cases, we put it in `tests/unit/mod.rs` and call it a day. However
// this crate doesn't have such file, so we put it here instead...
#[cfg(all(test, target_arch = "wasm32"))]
libparsec_tests_lite::platform::wasm_bindgen_test_configure!(run_in_browser);
