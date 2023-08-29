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
    ResponseMock, StatusCode,
};
// Re-expose
pub use libparsec_platform_http_proxy::ProxyConfig;
pub use libparsec_protocol as protocol;

/// We send the HTTP request with the body encoded in `msgpack` format.
/// This is the corresponding mime type to convey that info.
pub const PARSEC_CONTENT_TYPE: &str = "application/msgpack";

pub const API_VERSION_HEADER_NAME: &str = "Api-Version";
