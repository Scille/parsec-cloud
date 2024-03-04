// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

mod anonymous_cmds;
mod authenticated_cmds;
mod error;
mod invited_cmds;
#[cfg(feature = "test-with-testbed")]
mod testbed;

pub use anonymous_cmds::AnonymousCmds;
pub use authenticated_cmds::{
    sse::{RateLimiter, SSEConnectionError, SSEEvent, SSEResponseOrMissedEvents, SSEStream},
    AuthenticatedCmds, PARSEC_AUTH_METHOD,
};
pub use error::{ConnectionError, ConnectionResult};
pub use invited_cmds::InvitedCmds;
#[cfg(feature = "test-with-testbed")]
// Also re-expose reqwest&bytes stuff to simplify building mock response
pub use testbed::{
    test_register_low_level_send_hook, test_register_low_level_send_hook_default,
    test_register_low_level_send_hook_multicall, test_register_send_hook, Bytes, HeaderMap,
    HeaderValue, ResponseMock, StatusCode,
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

#[cfg(test)]
#[test]
fn user_agent_without_trailing_whitespace() {
    use libparsec_tests_lite::prelude::*;

    // It's easy to slip a \n when modifying the `../../../version` file
    p_assert_eq!(CLIENT_USER_AGENT.trim(), CLIENT_USER_AGENT);
    assert!(!CLIENT_USER_AGENT.contains('\t'), "{:?}", CLIENT_USER_AGENT);
    assert!(!CLIENT_USER_AGENT.contains('\n'), "{:?}", CLIENT_USER_AGENT);

    // User-Agent should be something like "Parsec-Client/0.0.0 Linux"
    let parts: Vec<_> = CLIENT_USER_AGENT.split_whitespace().collect();
    p_assert_eq!(parts.len(), 2, "{:?}", CLIENT_USER_AGENT);
}
