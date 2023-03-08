// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

mod authenticated_cmds;
mod client;
mod error;
mod invited_cmds;

pub use authenticated_cmds::{AuthenticatedCmds, PARSEC_AUTH_METHOD};
pub use client::{generate_client, generate_invited_client};
pub use error::{CommandError, CommandResult};
pub use invited_cmds::InvitedCmds;

/// We send the HTTP request with the body encoded in `msgpack` format.
/// This is the corresponding mime type to convey that info.
pub const PARSEC_CONTENT_TYPE: &str = "application/msgpack";

pub const API_VERSION_HEADER_NAME: &str = "Api-Version";
